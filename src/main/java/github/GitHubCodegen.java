package github;

import io.swagger.models.parameters.Parameter;
import io.swagger.models.parameters.SerializableParameter;
import io.swagger.models.parameters.BodyParameter;
import io.swagger.util.Json;

import io.swagger.codegen.v3.CliOption;
import io.swagger.codegen.v3.CodegenArgument;
import io.swagger.codegen.v3.CodegenConstants;
import io.swagger.codegen.v3.CodegenModel;
import io.swagger.codegen.v3.CodegenOperation;
import io.swagger.codegen.v3.CodegenParameter;
import io.swagger.codegen.v3.CodegenProperty;
import io.swagger.codegen.v3.CodegenResponse;
import io.swagger.codegen.v3.SupportingFile;
import io.swagger.codegen.v3.generators.DefaultCodegenConfig;
import io.swagger.codegen.v3.generators.handlebars.java.JavaHelper;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.PathItem;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.IntegerSchema;
import io.swagger.v3.oas.models.media.MapSchema;
import io.swagger.v3.oas.models.media.NumberSchema;
import io.swagger.v3.oas.models.media.ObjectSchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.StringSchema;
import io.swagger.v3.oas.models.responses.ApiResponse;
import io.swagger.v3.oas.models.parameters.RequestBody;
import io.swagger.v3.parser.util.SchemaTypeUtil;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.apache.commons.lang3.StringUtils;

import java.util.*;
import java.util.Map.Entry;
import static io.swagger.codegen.v3.generators.handlebars.ExtensionHelper.getBooleanValue;

public class GitHubCodegen extends RustServerCodegen {
    private static final Logger LOGGER = LoggerFactory.getLogger(GitHubCodegen.class);

    public GitHubCodegen() {
        super();

        supportingFiles.add(new SupportingFile("api_impl.rs", "src", "api_impl.rs"));
    }

    private static HashMap<String, Object> patchOperationBodyNames = new HashMap();
    private static HashMap<String, Object> patchOperationResponseNames = new HashMap();
    // Declare custom additions to inline enums that are behaving differently
    // than the official spec
    // private static HashMap<String, List<Map<String, String>>> patchEnumValues;
    // static {
    // patchEnumValues = new HashMap<String, List<Map<String, String>>>();
    // Map<String, String> additionalEnumValues = new HashMap<String, String>();
    // List<Map<String, String>> enumValues = new ArrayList <Map<String, String>>();

    // additionalEnumValues.put("name", "ROLLBACK_UPDATING");
    // additionalEnumValues.put("value", "\"rollback_updating\"");
    // enumValues.add(additionalEnumValues);

    // additionalEnumValues = new HashMap<String, String>();
    // additionalEnumValues.put("name", "ROLLBACK_PAUSED");
    // additionalEnumValues.put("value", "\"rollback_paused\"");
    // enumValues.add(additionalEnumValues);

    // additionalEnumValues = new HashMap<String, String>();
    // additionalEnumValues.put("name", "ROLLBACK_COMPLETED");
    // additionalEnumValues.put("value", "\"rollback_completed\"");
    // enumValues.add(additionalEnumValues);

    // patchEnumValues.put("ServiceUpdateStatusStateEnum", enumValues);
    // }

    @Override
    public void preprocessOpenAPI(OpenAPI openAPI) {
        Info info = openAPI.getInfo();
        List versionComponents = new ArrayList(Arrays.asList(info.getVersion().split("[.]")));
        while (versionComponents.size() < 3) {
            // Add the package version as a version component to the official specification
            // version
            versionComponents.add((String) additionalProperties.get(CodegenConstants.PACKAGE_VERSION));
        }

        info.setVersion(StringUtils.join(versionComponents, "."));

        super.preprocessOpenAPI(openAPI);
    }

    @Override
    public String getTypeDeclaration(Schema p) {
        String type = super.getTypeDeclaration(p);

        // This is a "fallback" type, and allows some parts of the Docker API
        // that receive an empty JSON '{}' value.
        if ("object".equals(type)) {
            type = "HashMap<(), ()>";
        }

        return type;
    }

    @Override
    public CodegenProperty fromProperty(String name, Schema p) {
        CodegenProperty property = super.fromProperty(name, p);

        // Remove extraneous references
        if (property.datatype.startsWith("models::")) {
            property.datatype = property.datatype.replace("models::", "");
        }

        return property;
    }

    @Override
    public Map<String, Object> postProcessAllModels(Map<String, Object> objs) {
        Map<String, Object> newObjs = super.postProcessAllModels(objs);

        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allModels = new HashMap<String, CodegenModel>();
        for (Entry<String, Object> entry : objs.entrySet()) {
            String modelName = toModelName(entry.getKey());
            Map<String, Object> inner = (Map<String, Object>) entry.getValue();
            List<Map<String, Object>> models = (List<Map<String, Object>>) inner.get("models");
            for (Map<String, Object> mo : models) {
                CodegenModel cm = (CodegenModel) mo.get("model");
                allModels.put(modelName, cm);
            }
        }

        for (Entry<String, CodegenModel> entry : allModels.entrySet()) {
            CodegenModel model = entry.getValue();
            if (model.getDataType() != null && model.getDataType().equals("boolean")) {
                model.vendorExtensions.put("x-rustgen-is-bool", true);
            }
            if (model.getIsEnum()) {
                model.vendorExtensions.put("is-enum", true);
            }
            for (CodegenProperty prop : model.vars) {
                if (prop.dataFormat != null && prop.dataFormat.equals("dateTime")) {
                    // set DateTime format on properties where appropriate
                    prop.datatype = "DateTime<Utc>";
                }

                if (getBooleanValue(prop, CodegenConstants.IS_ENUM_EXT_NAME)) {
                    ArrayList<HashMap<String, String>> vars = (ArrayList<HashMap<String, String>>) prop.allowableValues
                            .get("enumVars");
                    for (HashMap<String, String> enumVar : vars) {
                        String enumValue = enumVar.get("value");

                        // ensure we can deserialize inline enum values encoded as empty strings
                        if (enumValue != null && enumValue.length() <= 2) {
                            prop.vendorExtensions.put("x-rustgen-has-empty-enum", true);
                        }
                    }

                    // add additional enum values that get patched in at the template level
                    // if (patchEnumValues.containsKey(model.classname + prop.enumName)) {
                    // prop.vendorExtensions.put("x-rustgen-additional-enum-values",
                    // patchEnumValues.get(model.classname + prop.enumName));
                    // }
                }
            }
        }

        return newObjs;
    }

    @Override
    public Map<String, Object> postProcessOperationsWithModels(Map<String, Object> objs, List<Object> allModels) {
        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allTheModels = new HashMap<String, CodegenModel>();
        for (Entry<String, Object> entry : objs.entrySet()) {
            String modelName = toModelName(entry.getKey());
            if (entry.getValue() instanceof String) {
                String inner = (String) entry.getValue();
            } else if (entry.getValue() instanceof ArrayList) {
                List<Map<String, Object>> inner = (List<Map<String, Object>>) entry.getValue();
                for (Map<String, Object> mo : inner) {
                    // allTheModels.put(modelName, cm);
                    String className = (String) mo.get("import");
                    if (patchOperationBodyNames.get(className) != null) {
                        mo.put("import", patchOperationBodyNames.get(className));
                    }
                    if (patchOperationResponseNames.get(className) != null) {
                        mo.put("import", patchOperationResponseNames.get(className));
                    }
                }
            } else if (entry.getValue() instanceof Boolean) {
                Boolean inner = (Boolean) entry.getValue();
            } else {
                Map<String, Object> inner = (Map<String, Object>) entry.getValue();
            }
        }

        for (Object obj : (List<Object>) allModels) {
            Map<String, Object> map = (Map<String, Object>) obj;

            CodegenModel cm = (CodegenModel) map.get("model");
            String opName = (String) patchOperationBodyNames.get(camelize(cm.getName()));
            if (opName != null) {
                cm.setClassname(toModelName(opName));
            }
            String resName = (String) patchOperationResponseNames.get(camelize(cm.getName()));
            if (resName != null) {
                // LOGGER.info(" +++ setting resName " + camelize(cm.getName()) + " to " +
                // toModelName(resName));
                cm.setClassname(toModelName(resName));
            }
        }

        return objs;
    }

    @Override
    public void postProcessModelProperty(CodegenModel model, CodegenProperty property) {
        super.postProcessModelProperty(model, property);

        if (property.datatype.equals("isize")) {
            // needed for windows
            property.datatype = "i64";
        }
    }

    @Override
    public String toEnumVarName(String value, String datatype) {
        String name = super.toEnumVarName(value, datatype);
        if (name.length() == 0) {
            return "EMPTY";
        }
        return name;
    }

    @Override
    public CodegenParameter fromRequestBody(RequestBody body, String name, Schema schema, Map<String, Schema> schemas,
            Set<String> imports) {
        CodegenParameter param = super.fromRequestBody(body, name, schema, schemas, imports);

        // patches the Body* Models with better names
        if (body.getExtensions() != null) {
            Object operationName = body.getExtensions().get("x-codegen-operation-name");
            if (operationName != null && !operationName.toString().isEmpty() && name != null) {
                patchOperationBodyNames.put(camelize(name), operationName);
            }
        }

        return param;
    }

    @Override
    public CodegenResponse fromResponse(String responseCode, ApiResponse response) {
        CodegenResponse res = super.fromResponse(responseCode, response);

        if (response.getExtensions() != null) {
            Object operationName = response.getExtensions().get("x-codegen-operation-name");
            if (operationName != null && !operationName.toString().isEmpty() && res.getDataType() != null
                    && res.getDataType().startsWith("inline_response")) {
                patchOperationResponseNames.put(camelize(res.getDataType()),
                        operationName + "Response" + res.getCode());
                // LOGGER.info(" +++ res " + res + " datatype " + camelize(res.getDataType()) +
                // " patch " + operationName
                // + "Response" + res.getCode());
            }
        }

        return res;
    }

    @Override
    public String getSchemaType(Schema property) {
        String schemaType = super.getSchemaType(property);
        if (schemaType != null) {
            return schemaType.replace("UBUNTU", "ubuntu").replace("MACOS", "macos").replace("WINDOWS", "windows");
        } else {
            return null;
        }
    }
}
