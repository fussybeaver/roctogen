package github;

import static io.swagger.codegen.v3.generators.handlebars.ExtensionHelper.getBooleanValue;

import com.github.jknack.handlebars.Handlebars;
import io.swagger.codegen.v3.CodegenConstants;
import io.swagger.codegen.v3.CodegenModel;
import io.swagger.codegen.v3.CodegenOperation;
import io.swagger.codegen.v3.CodegenParameter;
import io.swagger.codegen.v3.CodegenProperty;
import io.swagger.codegen.v3.CodegenResponse;
import io.swagger.codegen.v3.SupportingFile;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.BooleanSchema;
import io.swagger.v3.oas.models.media.ComposedSchema;
import io.swagger.v3.oas.models.media.IntegerSchema;
import io.swagger.v3.oas.models.media.MapSchema;
import io.swagger.v3.oas.models.media.NumberSchema;
import io.swagger.v3.oas.models.media.ObjectSchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.StringSchema;
import io.swagger.v3.oas.models.parameters.RequestBody;
import io.swagger.v3.oas.models.responses.ApiResponse;

import java.util.*;
import java.util.Map.Entry;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class GitHubCodegen extends RustServerCodegen {

    private static final Logger LOGGER = LoggerFactory.getLogger(
            GitHubCodegen.class
    );

    protected Map<String, CodegenTag> tagList = new HashMap<
            String,
            CodegenTag
            >();
    protected Map<String, List<CodegenProperty>> mapLikeModels = new HashMap<
            String,
            List<CodegenProperty>
            >();

    public GitHubCodegen() {
        super();
        additionalProperties.put("tags", tagList.values());
        supportingFiles.add(
                new SupportingFile("lib.mustache", "src", "lib.rs")
        );
    }

    private static HashMap<String, Object> patchOperationBodyNames =
            new HashMap();
    private static HashMap<String, Object> patchOperationResponseNames =
            new HashMap();
    private static HashMap<
            String,
            HashMap<String, Object>
            > patchOneOfProperties = new HashMap();

    private static List<String> excludeBodyNames = Collections.emptyList();
    private static HashMap<String, String> patchResponseNames = new HashMap();

    @Override
    public void preprocessOpenAPI(OpenAPI openAPI) {
        Info info = openAPI.getInfo();
        List versionComponents = new ArrayList(
                Arrays.asList(info.getVersion().split("[.]"))
        );
        while (versionComponents.size() < 3) {
            // Add the package version as a version component to the official specification
            // version
            versionComponents.add(
                    (String) additionalProperties.get(
                            CodegenConstants.PACKAGE_VERSION
                    )
            );
        }

        info.setVersion(StringUtils.join(versionComponents, "."));

        super.preprocessOpenAPI(openAPI);
    }

    @Override
    public void processOpts() {
        super.processOpts();

        if (additionalProperties.containsKey("excludeBodyNames")) {
            excludeBodyNames = Arrays.asList(((String) additionalProperties.get("excludeBodyNames")).split("\\+"));
        }

        if (additionalProperties.containsKey("patchResponseNames")) {
            for (String ev : ((String) additionalProperties.get("patchResponseNames")).split("\\+")) {
                String[] names = ev.split(":", 2);
                if (names.length > 1) {
                    patchResponseNames.put(names[0], names[1]);
                }
            }
        }
    }

    @Override
    public String getTypeDeclaration(Schema p) {
        String type = super.getTypeDeclaration(p);

        // This is a "fallback" type, and allows some parts of the API
        // that receive an empty JSON '{}' value.
        if ("object".equals(type)) {
            type = "HashMap<String, Value>";
        }

        return type;
    }

    @Override
    public CodegenModel fromModel(
            String name,
            Schema schema,
            Map<String, Schema> allDefinitions
    ) {
        CodegenModel mdl = super.fromModel(name, schema, allDefinitions);

        // Partially deal with inline object polymorphism: 'anyOf' and 'oneOf'
        if (schema instanceof ComposedSchema) {
            ComposedSchema composedSchema = (ComposedSchema) schema;
            if (
                    composedSchema.getOneOf() != null ||
                            composedSchema.getAnyOf() != null
            ) {
                List<Schema> schemas;
                if (composedSchema.getOneOf() != null) {
                    schemas = composedSchema.getOneOf();
                } else {
                    schemas = composedSchema.getAnyOf();
                }
                mdl.getVendorExtensions().put("x-rustgen-enum-one-of", "true");

                int i = 0;

                Map<String, Object> allowableValues = new HashMap<
                        String,
                        Object
                        >();
                List<CodegenProperty> subModels = (List<
                        CodegenProperty
                        >) new ArrayList();
                allowableValues.put("count", schemas.size());
                Boolean allDisplayableTypes = true;

                for (Schema subSchema : schemas) {
                    String subName = name + "_sub_" + i;
                    CodegenProperty subMdl = fromProperty(subName, subSchema);
                    String type = getTypeDeclaration(subSchema);
                    if (subSchema instanceof ArraySchema) {
                        final ArraySchema arraySchema = (ArraySchema) subSchema;
                        Schema inner = arraySchema.getItems();
                        if (inner == null) {
                            LOGGER.warn(
                                    "warning!  No inner type supplied for array parameter \"" +
                                            subMdl.getName() +
                                            "\", using String"
                            );
                            inner = new StringSchema()
                                    .description(
                                            "//TODO automatically added by swagger-codegen"
                                    );
                            arraySchema.setItems(inner);
                        }

                        CodegenProperty item = fromProperty("inner", inner);
                        if (
                                item != null &&
                                        !getSchemaType(inner).equals("object")
                        ) {
                            item.setDatatype(toModelName(item.getDatatype()));
                        }
                        subMdl.items = item;
                        updatePropertyForArray(subMdl, item);

                        subMdl
                                .getVendorExtensions()
                                .put(
                                        CodegenConstants.IS_LIST_CONTAINER_EXT_NAME,
                                        Boolean.TRUE
                                );
                        subMdl
                                .getVendorExtensions()
                                .put(
                                        CodegenConstants.IS_CONTAINER_EXT_NAME,
                                        Boolean.TRUE
                                );

                        allDisplayableTypes = false;
                    } else if (subSchema.getProperties() != null) {
                        if (isObjectSchema(subSchema)) {
                            subMdl
                                    .getVendorExtensions()
                                    .put("x-is-object", Boolean.TRUE);
                        }

                        Map<String, Schema> properties =
                                subSchema.getProperties();
                        Iterator<Schema> values = properties
                                .values()
                                .iterator();

                        // We concern ourselves with the first type in an object - this is probably
                        // wrong for more complex definitions.
                        if (values.hasNext()) {
                            subMdl
                                    .getVendorExtensions()
                                    .put(
                                            CodegenConstants.IS_MAP_CONTAINER_EXT_NAME,
                                            Boolean.TRUE
                                    );
                            subMdl
                                    .getVendorExtensions()
                                    .put(
                                            CodegenConstants.IS_CONTAINER_EXT_NAME,
                                            Boolean.TRUE
                                    );
                            Schema innerSchema = values.next();
                            CodegenProperty cp = fromProperty(
                                    "inner",
                                    innerSchema
                            );

                            updatePropertyForMap(subMdl, cp);
                        }
                        allDisplayableTypes = false;
                    } else if (
                            !(subSchema instanceof NumberSchema) &&
                                    !(subSchema instanceof IntegerSchema) &&
                                    !(subSchema instanceof StringSchema) &&
                                    type != null
                    ) {
                        allDisplayableTypes = false;
                        subMdl.datatype = toModelName(type);
                    }

                    // Don't re-add a type that's a duplicate (the use of Value can mean we get
                    // dups)
                    Boolean containsDatatype = false;
                    for (CodegenProperty prop : subModels) {
                        if (prop.datatype != null && prop.datatype.equals(subMdl.datatype)) {
                            containsDatatype = true;
                        }
                    }

                    if (!containsDatatype) {
                        subModels.add(subMdl);
                        i++;
                    }
                }

                // Some enums are used when generating a url, so they need to implement std::Display
                if (allDisplayableTypes) {
                    mdl
                            .getVendorExtensions()
                            .put("x-rustgen-is-display", Boolean.TRUE);
                }

                mdl.setAllowableValues(allowableValues);
                allowableValues.put("values", subModels);
            }
        }

        return mdl;
    }

    @Override
    public CodegenProperty fromProperty(String name, Schema p) {
        CodegenProperty property = super.fromProperty(name, p);

        // Remove extraneous references
        if (property.datatype != null && property.datatype.startsWith("models::")) {
            property.datatype = property.datatype.replace("models::", "");
        }

        // Deal with Map-like Models
        if (p instanceof MapSchema) {
            MapSchema mp = (MapSchema) p;
            Object inner = mp.getAdditionalProperties();
            if (
                    !(inner instanceof Schema) &&
                            (Boolean) inner &&
                            mp.getProperties() != null &&
                            !mapLikeModels.containsKey(name)
            ) {
                Map<String, Schema> props = mp.getProperties();
                List<CodegenProperty> listProps = new ArrayList<
                        CodegenProperty
                        >();
                for (Entry<String, Schema> entry : props.entrySet()) {
                    CodegenProperty prop = fromProperty(
                            entry.getKey(),
                            entry.getValue()
                    );
                    if (
                            (prop.dataFormat == null ||
                                    !prop.dataFormat.equals("date-time")) &&
                                    !languageSpecificPrimitives.contains(prop.datatype)
                    ) {
                        prop.datatype = toModelName(prop.datatype);
                    }
                    listProps.add(prop);
                }
                mapLikeModels.put(name, listProps);
            }
        }

        // Deal with OneOf and AnyOf schemas in model properties.
        // We store the enum values as parseable untagged enums in a vendorExtension.
        // This currently only supports plain OneOf, AnyOf and Vectors of both.
        ComposedSchema composedSchema = null;
        if (p instanceof ArraySchema) {
            final ArraySchema arraySchema = (ArraySchema) p;
            Schema inner = arraySchema.getItems();
            if (inner instanceof ComposedSchema) {
                composedSchema = (ComposedSchema) inner;
            }
        }

        if (p instanceof ComposedSchema) {
            composedSchema = (ComposedSchema) p;
        }

        if (
                composedSchema != null &&
                        (composedSchema.getOneOf() != null ||
                                composedSchema.getAnyOf() != null)
        ) {
            List<Schema> schemas;
            if (composedSchema.getOneOf() != null) {
                schemas = composedSchema.getOneOf();
            } else {
                schemas = composedSchema.getAnyOf();
            }

            int i = 0;

            Map<String, Object> allowableValues = new HashMap<String, Object>();
            List<CodegenProperty> subModels = (List<
                    CodegenProperty
                    >) new ArrayList();
            allowableValues.put("count", schemas.size());

            for (Schema subSchema : schemas) {
                String subName = name + "_sub_" + i;
                CodegenProperty subMdl = fromProperty(subName, subSchema);
                String type = getTypeDeclaration(subSchema);

                if (
                        !(subSchema instanceof BooleanSchema) &&
                                !(subSchema instanceof ArraySchema) &&
                                !(subSchema instanceof ComposedSchema) &&
                                !(subSchema instanceof MapSchema) &&
                                !(subSchema instanceof NumberSchema) &&
                                !(subSchema instanceof IntegerSchema) &&
                                !(subSchema instanceof StringSchema) &&
                                type != null
                ) {
                    if (
                            isObjectSchema(subSchema) && !type.startsWith("HashMap")
                    ) {
                        subMdl.datatype = toModelName(type);
                    }
                }

                // Don't re-add a type that's a duplicate (the use of Value can mean we get
                // dups)
                Boolean containsDatatype = false;
                for (CodegenProperty prop : subModels) {
                    if (prop.datatype.equals(subMdl.datatype)) {
                        containsDatatype = true;
                    }
                }

                if (!containsDatatype) {
                    subModels.add(subMdl);
                    i++;
                }
            }

            allowableValues.put("values", subModels);
            property.vendorExtensions.put(
                    "x-codegen-one-of-schema",
                    allowableValues
            );
        }

        return property;
    }

    @Override
    public Map<String, Object> postProcessAllModels(Map<String, Object> objs) {
        Map<String, Object> newObjs = super.postProcessAllModels(objs);

        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allModels = new HashMap<
                String,
                CodegenModel
                >();
        for (Entry<String, Object> entry : objs.entrySet()) {
            String modelName = toModelName(entry.getKey());
            Map<String, Object> inner = (Map<String, Object>) entry.getValue();
            List<Map<String, Object>> models = (List<
                    Map<String, Object>
                    >) inner.get("models");
            for (Map<String, Object> mo : models) {
                CodegenModel cm = (CodegenModel) mo.get("model");
                allModels.put(modelName, cm);

                // Parse out OneOf and AnyOf enum values from a property. We need to do this here, because the affected models need to know their enum values.
                for (CodegenProperty prop : cm.vars) {
                    if (
                            prop.vendorExtensions.get("x-codegen-one-of-schema") !=
                                    null
                    ) {
                        if (prop.getItems() != null) {
                            patchOneOfProperties.put(
                                    prop.getItems().datatype,
                                    (HashMap<
                                            String,
                                            Object
                                            >) prop.vendorExtensions.get(
                                            "x-codegen-one-of-schema"
                                    )
                            );
                        } else {
                            patchOneOfProperties.put(
                                    prop.datatype,
                                    (HashMap<
                                            String,
                                            Object
                                            >) prop.vendorExtensions.get(
                                            "x-codegen-one-of-schema"
                                    )
                            );
                        }
                    }
                }
            }
        }

        for (Entry<String, CodegenModel> entry : allModels.entrySet()) {
            CodegenModel model = entry.getValue();
            if (
                    model.getDataType() != null &&
                            model.getDataType().equals("boolean")
            ) {
                model.vendorExtensions.put("x-rustgen-is-bool", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (
                    model.getDataType() != null &&
                            model.getDataType().equals("integer")
            ) {
                model.vendorExtensions.put("x-rustgen-is-integer", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (
                    model.getDataType() != null &&
                            model.getDataType().equals("String")
            ) {
                model.vendorExtensions.put("x-rustgen-is-string", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (
                    model.getDataType() != null &&
                            model.getDataType().equals("DateTime")
            ) {
                model.vendorExtensions.put("x-rustgen-is-datetime", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.arrayModelType != null) {
                model.vendorExtensions.put("x-rustgen-is-array", true);
                model.vendorExtensions.put("has-vars", true);
            }
            if (model.getIsEnum()) {
                model.vendorExtensions.put("is-enum", true);
            }

            // Patch in the enum values for a OneOf or AnyOf schema found in a previous model property.
            if (
                    model.classname.startsWith("OneOf") ||
                            model.classname.startsWith("AnyOf")
            ) {
                if (patchOneOfProperties.containsKey(model.classname)) {
                    model.allowableValues = patchOneOfProperties.get(
                            model.classname
                    );
                    model
                            .getVendorExtensions()
                            .put("x-rustgen-enum-one-of", "true");
                } else {
                    newObjs.remove(model.classname);
                }
            }

            for (CodegenProperty prop : model.vars) {
                if (
                        prop.dataFormat != null &&
                                prop.dataFormat.equals("dateTime")
                ) {
                    // set DateTime format on properties where appropriate
                    prop.datatype = "DateTime<Utc>";
                }

                if (getBooleanValue(prop, CodegenConstants.IS_ENUM_EXT_NAME)) {
                    ArrayList<HashMap<String, String>> vars = (ArrayList<
                            HashMap<String, String>
                            >) prop.allowableValues.get("enumVars");
                    for (HashMap<String, String> enumVar : vars) {
                        String enumValue = enumVar.get("value");

                        // ensure we can deserialize inline enum values encoded as empty strings
                        if (enumValue != null && enumValue.length() <= 2) {
                            prop.vendorExtensions.put(
                                    "x-rustgen-has-empty-enum",
                                    true
                            );
                        }
                    }
                }

                if (prop.baseName.equals(prop.name)) {
                    prop.vendorExtensions.put(
                            "x-rustgen-serde-no-rename",
                            true
                    );
                }
                if (prop.datatype != null && prop.datatype.equals("String")) {
                    prop.vendorExtensions.put("x-rustgen-is-string", true);
                    model.vendorExtensions.put("x-rustgen-has-string", true);
                }

                if (
                        prop.baseName.equals("score") && prop.datatype.equals("i64")
                ) {
                    prop.datatype = "f32";
                }
            }

            if (mapLikeModels.containsKey(model.name)) {
                model.vars = mapLikeModels.get(model.name);
            }
        }

        return newObjs;
    }

    @Override
    public void addOperationToGroup(
            String tag,
            String resourcePath,
            Operation operation,
            CodegenOperation co,
            Map<String, List<CodegenOperation>> operations
    ) {
        super.addOperationToGroup(tag, resourcePath, operation, co, operations);
        CodegenTag codegenTag;
        if (tagList.containsKey(tag)) {
            codegenTag = tagList.get(tag);
        } else {
            codegenTag = new CodegenTag();
            codegenTag.baseName = underscore(tag);
            codegenTag.classname = tag;
            codegenTag.operations = new ArrayList<CodegenOperation>();
            tagList.put(tag, codegenTag);
        }

        List<CodegenOperation> codegenOperations = codegenTag.getOperations();
        codegenOperations.add(co);
        codegenTag.setContents(co.getContents());
    }

    interface PostProcessLambda {
        public boolean existsInModels(String a);
    }

    @Override
    public Map<String, Object> postProcessOperationsWithModels(
            Map<String, Object> objs,
            List<Object> allModels
    ) {
        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allTheModels = new HashMap<
                String,
                CodegenModel
                >();

        for (Object obj : (List<Object>) allModels) {
            Map<String, Object> map = (Map<String, Object>) obj;

            CodegenModel cm = (CodegenModel) map.get("model");

            String opName = (String) patchOperationBodyNames.get(
                    camelize(cm.getName())
            );
            if (opName != null && !cm.name.contains(" ")) {
                cm.setClassname(toModelName(opName));
                cm.getVendorExtensions().put("x-rustgen-body-model", "true");
            }
            String resName = (String) patchOperationResponseNames.get(
                    camelize(cm.getName())
            );
            if (resName != null) {
                cm.setClassname(toModelName(resName));
            }

            // Sanitize OneOf and AnyOf names, replace the /body[0-9]+/ naming with nicer names stored previously.
            Pattern reBody = Pattern.compile("(^Body[0-9]+)");

            // Ensure that we do not rename an Allof* or Oneof* to a colliding modelname
            PostProcessLambda p = replacement -> {
                return allTheModels
                        .values()
                        .stream()
                        .noneMatch(obj2 -> {
                            CodegenModel cm2 = (CodegenModel) ((Map<
                                    String,
                                    Object
                                    >) obj2).get("model");
                            if (cm2.classname.equals(replacement)) {
                                return true;
                            }
                            return false;
                        });
            };

            // Rename models that are prefixed with an Allof or Oneof
            if (
                    (cm.classname.startsWith("OneOf") ||
                            cm.classname.startsWith("AnyOf")) &&
                            !p.existsInModels(
                                    cm.classname
                                            .replaceFirst("OneOf", "")
                                            .replaceFirst("AnyOf", "")
                            )
            ) {
                cm.classname = camelize(cm.classname.replaceFirst("OneOf", ""));
                cm.classname = camelize(cm.classname.replaceFirst("AnyOf", ""));
            }
            Matcher matchBody = reBody.matcher(cm.classname);

            // Rename property types that refer to an Allof or Oneof model
            for (CodegenProperty property : cm.vars) {
                if (property.datatype != null) {

                    if (property.datatype.startsWith("OneOf")) {
                        property.datatype = camelize(
                                p.existsInModels(
                                        property.datatype.replaceFirst("OneOf", "")
                                )
                                        ? property.datatype
                                        : property.datatype.replaceFirst("OneOf", "")
                        );
                    } else if (property.datatype.startsWith("AnyOf")) {
                        property.datatype = camelize(
                                p.existsInModels(
                                        property.datatype.replaceFirst("AnyOf", "")
                                )
                                        ? property.datatype
                                        : property.datatype.replaceFirst("AnyOf", "")
                        );
                    }
                }

                if (
                        property.getItems() != null && property.getItems().datatype != null &&
                                property.getItems().datatype.startsWith("OneOf")
                ) {
                    property.getItems().datatype = camelize(
                            p.existsInModels(
                                    property
                                            .getItems()
                                            .datatype.replace("OneOf", "")
                            )
                                    ? property.getItems().datatype
                                    : property.getItems().datatype.replace("OneOf", "")
                    );
                } else if (
                        property.getItems() != null && property.getItems().datatype != null &&
                                property.getItems().datatype.startsWith("AnyOf")
                ) {
                    property.getItems().datatype = camelize(
                            p.existsInModels(
                                    property
                                            .getItems()
                                            .datatype.replace("AnyOf", "")
                            )
                                    ? property.getItems().datatype
                                    : property.getItems().datatype.replace("AnyOf", "")
                    );
                }
            }


            if (matchBody.find()) {
                String body = matchBody.group(0);

                if (patchOperationBodyNames.containsKey(body)) {
                    cm.classname = cm.classname.replace(
                            body,
                            camelize((String) patchOperationBodyNames.get(body))
                    ) +
                            "Enum";
                }
            }

            for (CodegenProperty property : cm.vars) {
                if (property.datatype != null) {
                    Matcher matchProp = reBody.matcher(property.datatype);
                    if (matchProp.find()) {
                        property.datatype = property.datatype.replace(
                                matchProp.group(0),
                                camelize(
                                        (String) patchOperationBodyNames.get(
                                                matchProp.group(0)
                                        )
                                )
                        ) +
                                "Enum";
                    }
                }

                if (property.getItems() != null && property.getItems().datatype != null) {
                    Matcher matchProp = reBody.matcher(property.getItems().datatype);

                    if (matchProp.find()) {
                        property.getItems().datatype = property
                                .getItems()
                                .datatype.replace(
                                        matchProp.group(0),
                                        camelize(
                                                (String) patchOperationBodyNames.get(
                                                        matchProp.group(0)
                                                )
                                        )
                                ) +
                                "Enum";
                    }
                }
            }
        }
        Map<String, Object> operations = (Map<String, Object>) objs.get(
                "operations"
        );
        if (operations != null) {
            List<CodegenOperation> ops = (List<
                    CodegenOperation
                    >) operations.get("operation");
            for (final CodegenOperation operation : ops) {
                List<CodegenResponse> responses = operation.getResponses();
                Boolean hasDefaultResponse = false;
                for (final CodegenResponse res : responses) {
                    if (res.getDataType() != null) {
                        if (
                                getBooleanValue(
                                        res,
                                        CodegenConstants.IS_DEFAULT_EXT_NAME
                                )
                        ) {
                            hasDefaultResponse = true;
                        }
                        String resName =
                                (String) patchOperationResponseNames.get(
                                        camelize(res.getDataType())
                                );


                        if (resName != null) {
                            if (res.getDataType().contains("InlineResponse") || allModels.stream().anyMatch(obj2 -> {
                                CodegenModel cm2 = (CodegenModel) ((Map<String, Object>) obj2).get("model");
                                if (cm2.classname.equals(resName)) {
                                    return true;
                                }
                                return false;
                            })) {
                                res.dataType = toModelName(resName);
                            }

                        }
                    }
                }
                if (!hasDefaultResponse) {
                    operation
                            .getVendorExtensions()
                            .put("x-codegen-response-empty-default", "true");
                }
            }
        }


        return objs;
    }

    @Override
    public Map<String, Object> postProcessOperations(Map<String, Object> objs) {
        objs = super.postProcessOperations(objs);
        Map<String, Object> operations = (Map<String, Object>) objs.get(
                "operations"
        );
        if (operations != null) {
            List<CodegenOperation> ops = (List<
                    CodegenOperation
                    >) operations.get("operation");
            for (final CodegenOperation operation : ops) {
                if (operation.notes != null) {
                    operation.unescapedNotes =
                            operation.unescapedNotes.replaceAll(
                                    "```(.+)\\n(.+)\\n",
                                    "```$1,nocompile\n$2\n"
                            );
                    operation.unescapedNotes =
                            operation.unescapedNotes.replaceAll(
                                    "```\\n(.+)\\n",
                                    "```nocompile\n$1\n"
                            );
                    operation.unescapedNotes =
                            operation.unescapedNotes.replaceAll(
                                    "\\n",
                                    "\n    /// "
                            );
                }

                CodegenParameter body = operation.bodyParam;
                if (body != null) {
                    String opName = (String) patchOperationBodyNames.get(
                            camelize(body.getDataType())
                    );
                    if (opName != null) {
                        body.dataType = toModelName(opName);
                    }
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
                    if (
                            getBooleanValue(
                                    param,
                                    CodegenConstants.IS_STRING_EXT_NAME
                            ) ||
                                    getBooleanValue(
                                            param,
                                            CodegenConstants.IS_UUID_EXT_NAME
                                    )
                    ) {
                        hasStringParams = true;
                    }
                }
                if (hasPerPage && hasPage) {
                    operation
                            .getVendorExtensions()
                            .put("x-codegen-impl-per-page", "true");
                }
                if (hasOptionalQueryParams) {
                    operation
                            .getVendorExtensions()
                            .put("x-codegen-has-optional-query-params", "true");
                }
                if (hasStringParams) {
                    operation
                            .getVendorExtensions()
                            .put("x-codegen-has-string-params", "true");
                }

                if (operation.getVendorExtensions().get("x-github") != null) {
                    Map<String, Object> xgh = (Map<String, Object>) operation
                            .getVendorExtensions()
                            .get("x-github");
                    if (xgh.get("previews") != null) {
                        List<Map<String, String>> previews = (List<
                                Map<String, String>
                                >) xgh.get("previews");
                        if (!previews.isEmpty()) {
                            operation
                                    .getVendorExtensions()
                                    .put("x-codegen-has-previews", "true");
                        }
                        for (final Map<String, String> pre : previews) {
                            LOGGER.debug(
                                    "GitHub Preview token: " + pre.get("name")
                            );
                        }
                    }
                }
            }
        }
        return objs;
    }

    @Override
    public void postProcessModelProperty(
            CodegenModel model,
            CodegenProperty property
    ) {
        super.postProcessModelProperty(model, property);

        if (property.datatype != null && property.datatype.equals("isize")) {
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
    public CodegenParameter fromRequestBody(
            RequestBody body,
            String name,
            Schema schema,
            Map<String, Schema> schemas,
            Set<String> imports
    ) {
        CodegenParameter param = super.fromRequestBody(
                body,
                name,
                schema,
                schemas,
                imports
        );

        // patches the Body* Models with better names - injected through additionalProperties
        if (body.getExtensions() != null) {
            Object operationName = body
                    .getExtensions()
                    .get("x-codegen-operation-name");
            if (
                    operationName != null &&
                            !operationName.toString().isEmpty() &&
                            name != null
            ) {
                if (!excludeBodyNames.contains(camelize(name))) {
                    patchOperationBodyNames.put(camelize(name), operationName);
                }
            }
        }

        // Needed because of the static call to `::from_json`, which requires an unqualified type
        if (
                schema instanceof ObjectSchema &&
                        param.getDataType().startsWith("HashMap")
        ) {
            param
                    .getVendorExtensions()
                    .put(CodegenConstants.IS_MAP_CONTAINER_EXT_NAME, Boolean.TRUE);
            param
                    .getVendorExtensions()
                    .put(CodegenConstants.IS_CONTAINER_EXT_NAME, Boolean.TRUE);
        }

        // Needed for uploads and raw bytes
        if (param.getDataType().startsWith("Object")) {
            param
                    .getVendorExtensions()
                    .put("x-codegen-body-bytes", Boolean.TRUE);
            param.dataType = "std::vec::Vec<u8>";
        }

        return param;
    }

    @Override
    public void addParentContainer(
            CodegenModel codegenModel,
            String name,
            Schema schema
    ) {
        super.addParentContainer(codegenModel, name, schema);
    }

    @Override
    public CodegenResponse fromResponse(
            String responseCode,
            ApiResponse response
    ) {
        CodegenResponse res = super.fromResponse(responseCode, response);

        if (response.getExtensions() != null) {
            Object operationName = response
                    .getExtensions()
                    .get("x-codegen-operation-name");

            // Replace InlineResponse models with a proper name
            if (
                    operationName != null &&
                            !operationName.toString().isEmpty() &&
                            res.getDataType() != null
            ) {
                if (
                        res.getDataType().startsWith("InlineResponse")

                ) {
                    patchOperationResponseNames.put(
                            camelize(res.getDataType()),
                            operationName + "Response" + res.getCode()
                    );
                }
            }
        }

        // Special cases injected through additionalProperties
        for (Entry entry : patchResponseNames.entrySet()) {
            if (
                    res.getDataType() != null &&
                            res.getDataType().equals(entry.getKey())
            ) {
                res.dataType = (String) entry.getValue();
                patchOperationResponseNames.put(res.getDataType(), entry.getValue());
            }
        }

        return res;
    }

    @Override
    public String getSchemaType(Schema property) {
        String schemaType = super.getSchemaType(property);
        if (schemaType != null) {
            return schemaType
                    .replace("UBUNTU", "ubuntu")
                    .replace("MACOS", "macos")
                    .replace("WINDOWS", "windows");
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
