package github;

import static io.swagger.codegen.v3.generators.handlebars.ExtensionHelper.getBooleanValue;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Set;

import com.github.jknack.handlebars.Handlebars;

import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.swagger.codegen.v3.CodegenConstants;
import io.swagger.codegen.v3.CodegenModel;
import io.swagger.codegen.v3.CodegenOperation;
import io.swagger.codegen.v3.CodegenParameter;
import io.swagger.codegen.v3.CodegenProperty;
import io.swagger.codegen.v3.CodegenResponse;
import io.swagger.codegen.v3.generators.handlebars.java.JavaHelper;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.parameters.RequestBody;
import io.swagger.v3.oas.models.responses.ApiResponse;

public class GitHubCodegen extends RustServerCodegen {
    private static final Logger LOGGER = LoggerFactory.getLogger(GitHubCodegen.class);

    protected Map<String, CodegenTag> tagList = new HashMap<String, CodegenTag>();

    public GitHubCodegen() {
        super();

        additionalProperties.put("tags", tagList.values());
        // supportingFiles.add(new SupportingFile("api_impl.rs", "src", "api_impl.rs"));
    }

    private static HashMap<String, Object> patchOperationBodyNames = new HashMap();
    private static HashMap<String, Object> patchOperationResponseNames = new HashMap();

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
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getDataType() != null && model.getDataType().equals("integer")) {
                model.vendorExtensions.put("x-rustgen-is-integer", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getDataType() != null && model.getDataType().equals("String")) {
                model.vendorExtensions.put("x-rustgen-is-string", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getDataType() != null && model.getDataType().equals("DateTime")) {
                model.vendorExtensions.put("x-rustgen-is-datetime", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getClassname().equals("WorkflowId")) {
                model.vendorExtensions.put("x-rustgen-is-untagged-enum", true);
                model.vendorExtensions.put("is-enum", true);
                model.allowableValues = new HashMap<String, Object>();

                List<Map<String, String>> enumVars = new ArrayList<>();
                Map<String, String> intEnumVar = new HashMap<String, String>();
                intEnumVar.put("name", "Int");
                intEnumVar.put("value", "i32");
                enumVars.add(intEnumVar);
                Map<String, String> strEnumVar = new HashMap<String, String>();
                strEnumVar.put("name", "Str");
                strEnumVar.put("value", "String");
                enumVars.add(strEnumVar);
                model.allowableValues.put("enumVars", enumVars);

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
                
                if (prop.baseName.equals(prop.name)) {
                    prop.vendorExtensions.put("x-rustgen-serde-no-rename", true);
                }
                if (prop.datatype != null && prop.datatype.equals("String")) {
                    prop.vendorExtensions.put("x-rustgen-is-string", true);
                }
                
                if (prop.baseName.equals("score") && prop.datatype.equals("i64")) {
                    prop.datatype = "f32";
                }
            }
            //if (model.readWriteVars != null) {
            //    LOGGER.info("::: readOnlyVars " + model.readWriteVars);
            //}
        }

        return newObjs;
    }

    @Override
    public void addOperationToGroup(String tag, String resourcePath, Operation operation, CodegenOperation co,
            Map<String, List<CodegenOperation>> operations) {

        super.addOperationToGroup(tag, resourcePath, operation, co, operations);
        CodegenTag codegenTag;
        if (tagList.containsKey(tag)) {
            codegenTag = tagList.get(tag);
        } else {
            codegenTag = new CodegenTag();
            codegenTag.baseName = underscore(tag);
            codegenTag.operations = new ArrayList<CodegenOperation>();
            tagList.put(tag, codegenTag);
        }

        List<CodegenOperation> codegenOperations = codegenTag.getOperations();
        codegenOperations.add(co);
        codegenTag.setContents(co.getContents());
    }

    @Override
    public Map<String, Object> postProcessOperationsWithModels(Map<String, Object> objs, List<Object> allModels) {
        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allTheModels = new HashMap<String, CodegenModel>();
        // for (Entry<String, Object> entry : objs.entrySet()) {
        // String modelName = toModelName(entry.getKey());
        // if (entry.getValue() instanceof String) {
        // String inner = (String) entry.getValue();
        // } else if (entry.getValue() instanceof ArrayList) {
        // List<Map<String, Object>> inner = (List<Map<String, Object>>)
        // entry.getValue();
        // for (Map<String, Object> mo : inner) {
        // // allTheModels.put(modelName, cm);
        // String className = (String) mo.get("import");
        // if (patchOperationBodyNames.get(className) != null) {
        // mo.put("import", patchOperationBodyNames.get(className));
        // }
        // if (patchOperationResponseNames.get(className) != null) {
        // mo.put("import", patchOperationResponseNames.get(className));
        // }
        // }
        // } else if (entry.getValue() instanceof Boolean) {
        // Boolean inner = (Boolean) entry.getValue();
        // } else {
        // Map<String, Object> inner = (Map<String, Object>) entry.getValue();
        // }
        // }

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
    public Map<String, Object> postProcessOperations(Map<String, Object> objs) {
        objs = super.postProcessOperations(objs);
        Map<String, Object> operations = (Map<String, Object>) objs.get("operations");
        if (operations != null) {
            List<CodegenOperation> ops = (List<CodegenOperation>) operations.get("operation");
            for (final CodegenOperation operation : ops) {
                if (operation.notes != null) {
                    operation.unescapedNotes = operation.unescapedNotes.replaceAll("```(.+)\\n(.+)\\n", "```$1,nocompile\n$2\n");
                    operation.unescapedNotes = operation.unescapedNotes.replaceAll("```\\n(.+)\\n", "```nocompile\n$1\n");
                    operation.unescapedNotes = operation.unescapedNotes.replaceAll("\\n", "\n    /// ");

                }

                CodegenParameter body = operation.bodyParam;
                if (body != null) {
                    String opName = (String) patchOperationBodyNames.get(camelize(body.getDataType()));
                    if (opName != null) {
                        body.dataType = toModelName(opName);
                    }
                }
                List<CodegenResponse> responses = operation.getResponses();
                Boolean hasDefaultResponse = false;
                for (final CodegenResponse res : responses) {
                    if (res.getDataType() != null) {
                        if (getBooleanValue(res, CodegenConstants.IS_DEFAULT_EXT_NAME)) {
                            hasDefaultResponse = true;
                        }
                        String resName = (String) patchOperationResponseNames.get(camelize(res.getDataType()));
                        if (resName != null) {
                            res.dataType = toModelName(resName);
                        }
                    }
                }
                if (!hasDefaultResponse) {
                    operation.getVendorExtensions().put("x-codegen-response-empty-default", "true");
                }
                List<CodegenParameter> queryParams = operation.queryParams;
                Boolean hasPerPage = false;
                Boolean hasPage = false;
                Boolean hasOptionalQueryParams = true;
                Boolean hasStringParams = false;
                for (final CodegenParameter param : queryParams) {
                    if (param.getParamName().equals("per_page")) {
                        hasPerPage = true;
                        param.dataType = "u16";
                        param.getVendorExtensions().put("x-is-string", "false");
                    }
                    if (param.getParamName().equals("page")) {
                        hasPage = true;
                        param.dataType = "u16";
                        param.getVendorExtensions().put("x-is-string", "false");
                    }
                    if (param.getRequired()) {
                        hasOptionalQueryParams = false;
                    }
                    if (getBooleanValue(param, CodegenConstants.IS_STRING_EXT_NAME) || getBooleanValue(param, CodegenConstants.IS_UUID_EXT_NAME)) {
                        hasStringParams = true;
                    }
                }
                if (hasPerPage && hasPage) {
                    operation.getVendorExtensions().put("x-codegen-impl-per-page", "true");
                }
                if (hasOptionalQueryParams) {
                    operation.getVendorExtensions().put("x-codegen-has-optional-query-params", "true");
                }
                if (hasStringParams) {
                    operation.getVendorExtensions().put("x-codegen-has-string-params", "true");
                }

                if (operation.getVendorExtensions().get("x-github") != null) {
                    Map<String, Object> xgh = (Map<String, Object>) operation.getVendorExtensions().get("x-github");
                    if (xgh.get("previews") != null) {
                        List<Map<String, String>> previews = (List<Map<String, String>>) xgh.get("previews");
                        if (!previews.isEmpty()) {
                            operation.getVendorExtensions().put("x-codegen-has-previews", "true");
                        }
                        for (final Map<String, String> pre : previews) {
                            LOGGER.debug("GitHub Preview token: " + pre.get("name"));
                        }
                    }
                }
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
                    && res.getDataType().startsWith("InlineResponse")) {
                patchOperationResponseNames.put(camelize(res.getDataType()),
                        operationName + "Response" + res.getCode());
            }
        }
        
        // Special case
        if (res.getDataType() != null && res.getDataType().equals("SelectedActions")) {
            res.dataType = "PutActionsSetAllowedActionsRepository";
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

    @Override
    public String toOperationId(String operationId) {
        operationId = operationId.replaceFirst("[a-zA-Z0-9]+\\/", "");

        return super.toOperationId(operationId);
    }
    
    @Override
    public void addHandlebarHelpers(Handlebars handlebars) {
        super.addHandlebarHelpers(handlebars);
        handlebars.registerHelpers(new IfCondHelper());
    }

    @Override
    public String toVarName(String name) {
        if (name.equals("ref")) {
            return "git_ref";
        }
        
        return super.toVarName(name);
    }

}
