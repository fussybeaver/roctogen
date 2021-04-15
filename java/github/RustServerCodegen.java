package github;

import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import io.swagger.codegen.v3.CliOption;
import io.swagger.codegen.v3.CodegenConstants;
import io.swagger.codegen.v3.CodegenModel;
import io.swagger.codegen.v3.CodegenOperation;
import io.swagger.codegen.v3.CodegenParameter;
import io.swagger.codegen.v3.CodegenProperty;
import io.swagger.codegen.v3.CodegenResponse;
import io.swagger.codegen.v3.CodegenType;
import io.swagger.codegen.v3.SupportingFile;
import io.swagger.codegen.v3.generators.DefaultCodegenConfig;
import io.swagger.codegen.v3.generators.util.OpenAPIUtil;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.BinarySchema;
import io.swagger.v3.oas.models.media.BooleanSchema;
import io.swagger.v3.oas.models.media.ComposedSchema;
import io.swagger.v3.oas.models.media.FileSchema;
import io.swagger.v3.oas.models.media.IntegerSchema;
import io.swagger.v3.oas.models.media.MapSchema;
import io.swagger.v3.oas.models.media.NumberSchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.StringSchema;
import io.swagger.v3.oas.models.parameters.Parameter;
import io.swagger.v3.oas.models.parameters.RequestBody;
import io.swagger.v3.oas.models.responses.ApiResponse;
import io.swagger.v3.parser.util.SchemaTypeUtil;

public class RustServerCodegen extends DefaultCodegenConfig {
    private static final Logger LOGGER = LoggerFactory.getLogger(RustServerCodegen.class);

    private static final String NO_FORMAT = "%%NO_FORMAT";

    protected String apiVersion = "1.0.0";
    protected String projectName = "swagger-client";
    protected String apiPath = "src/endpoints";
    protected String packageName;
    protected String packageVersion;
    protected String externCrateName;
    protected Map<String, Map<String, String>> pathSetMap = new HashMap<String, Map<String, String>>();

    public RustServerCodegen() {
        super();

        this.supportsMixins = false;
        this.copyFistAllOfProperties = true;
        super.supportsMixins = false;
        super.copyFistAllOfProperties = true;

        // set the output folder here
        outputFolder = "generated-code/rust-client";

        /*
         * Models. You can write model files using the modelTemplateFiles map. if you
         * want to create one template for file, you can do so here. for multiple files
         * for model, just put another entry in the `modelTemplateFiles` with a
         * different extension
         */
        // modelTemplateFiles.put("models.mustache", ".rs");

        /*
         * Api classes. You can write classes for each Api file with the
         * apiTemplateFiles map. as with models, add multiple entries with different
         * extensions for multiple files per class
         */
        apiTemplateFiles.put("api.mustache", ".rs");

        /*
         * Template Location. This is the location which templates will be read from.
         * The generator will use the resource stream to attempt to read the templates.
         */
        embeddedTemplateDir = templateDir = "rust-client";

        /*
         * Reserved words. Override this with reserved words specific to your language
         */
        setReservedWordsLowerCase(Arrays.asList(
                // From https://doc.rust-lang.org/grammar.html#keywords
                "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do", "else",
                "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match",
                "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub", "pure", "ref", "return", "Self",
                "self", "sizeof", "static", "struct", "super", "trait", "true", "type", "typeof", "unsafe", "unsized",
                "use", "virtual", "where", "while", "yield"));

        defaultIncludes = new HashSet<String>(Arrays.asList("map", "array"));

        languageSpecificPrimitives = new HashSet<String>(Arrays.asList("bool", "char", "i8", "i16", "i32", "i64", "u8",
                "u16", "u32", "u64", "isize", "usize", "f32", "f64", "str"));

        instantiationTypes.clear();
        instantiationTypes.put("array", "Vec");
        instantiationTypes.put("map", "Map");

        typeMapping.clear();
        typeMapping.put("number", "f64");
        typeMapping.put("integer", "i32");
        typeMapping.put("long", "i64");
        typeMapping.put("float", "f32");
        typeMapping.put("double", "f64");
        typeMapping.put("BigDecimal", "f64");
        typeMapping.put("string", "String");
        typeMapping.put("UUID", "uuid::Uuid");
        typeMapping.put("byte", "u8");
        typeMapping.put("ByteArray", "swagger::ByteArray");
        typeMapping.put("binary", "swagger::ByteArray");
        typeMapping.put("boolean", "bool");
        typeMapping.put("date", "chrono::DateTime<chrono::Utc>");
        typeMapping.put("DateTime", "chrono::DateTime<chrono::Utc>");
        typeMapping.put("password", "String");
        typeMapping.put("File", "Box<Stream<Item=Vec<u8>, Error=Error> + Send>");
        typeMapping.put("file", "Box<Stream<Item=Vec<u8>, Error=Error> + Send>");
        typeMapping.put("array", "Vec");
        typeMapping.put("map", "HashMap");
        typeMapping.put("Object", "Value");

        importMapping = new HashMap<String, String>();

        cliOptions.clear();
        cliOptions.add(new CliOption(CodegenConstants.PACKAGE_NAME, "Rust crate name (convention: snake_case).")
                .defaultValue("swagger_client"));
        cliOptions.add(new CliOption(CodegenConstants.PACKAGE_VERSION, "Rust crate version.").defaultValue("1.0.0"));

        /*
         * Additional Properties. These values can be passed to the templates and are
         * available in models, apis, and supporting files
         */
        additionalProperties.put("apiVersion", apiVersion);
        additionalProperties.put("apiPath", apiPath);

        /*
         * Supporting Files. You can write single files for the generator with the
         * entire object tree available. If the input file has a suffix of `.mustache it
         * will be processed by the template engine. Otherwise, it will be copied
         */
        supportingFiles.add(new SupportingFile("models.mustache", "src", "models.rs"));
        supportingFiles.add(new SupportingFile("endpoints.mustache", "src/endpoints", "mod.rs"));

    }

    @Override
    public void processOpts() {
        super.processOpts();

        if (additionalProperties.containsKey(CodegenConstants.PACKAGE_NAME)) {
            setPackageName((String) additionalProperties.get(CodegenConstants.PACKAGE_NAME));
        } else {
            setPackageName("swagger_client");
        }

        if (additionalProperties.containsKey(CodegenConstants.PACKAGE_VERSION)) {
            setPackageVersion((String) additionalProperties.get(CodegenConstants.PACKAGE_VERSION));
        } else {
            setPackageVersion("1.0.0");
        }

        additionalProperties.put(CodegenConstants.PACKAGE_NAME, packageName);
        additionalProperties.put(CodegenConstants.PACKAGE_VERSION, packageVersion);
        additionalProperties.put("externCrateName", externCrateName);
    }

    public void setPackageName(String packageName) {
        this.packageName = packageName;

        // Also set the extern crate name, which has any '-' replace with a '_'.
        this.externCrateName = packageName.replace('-', '_');
    }

    public void setPackageVersion(String packageVersion) {
        this.packageVersion = packageVersion;
    }

    @Override
    public String apiPackage() {
        return apiPath;
    }

    @Override
    public void preprocessOpenAPI(OpenAPI openapi) {
        super.preprocessOpenAPI(openapi);

        Info info = openapi.getInfo();
        List versionComponents = new ArrayList(Arrays.asList(info.getVersion().split("[.]")));
        if (versionComponents.size() < 1) {
            versionComponents.add("1");
        }
        while (versionComponents.size() < 3) {
            versionComponents.add("0");
        }
        info.setVersion(StringUtils.join(versionComponents, "."));

    }

    @Override
    public String toApiName(String name) {
        if (name.length() == 0) {
            return "default";
        }
        return camelize(name);
    }

    /**
     * Escapes a reserved word as defined in the `reservedWords` array. Handle
     * escaping those terms here. This logic is only called if a variable matches
     * the reserved words
     *
     * @return the escaped term
     */
    @Override
    public String escapeReservedWord(String name) {
        if (this.reservedWordsMappings().containsKey(name)) {
            return this.reservedWordsMappings().get(name);
        }
        return "_" + name; // add an underscore to the name
    }

    /**
     * Location to write api files. You can use the apiPackage() as defined when the
     * class is instantiated
     */
    @Override
    public String apiFileFolder() {
        return outputFolder + File.separator + apiPackage().replace('.', File.separatorChar);
    }

    @Override
    public String toModelName(String name) {
        // camelize the model name
        // phone_number => PhoneNumber
        String camelizedName = camelize(toModelFilename(name));

        // model name cannot use reserved keyword, e.g. return
        if (isReservedWord(camelizedName)) {
            camelizedName = "Model" + camelizedName;
            LOGGER.warn(camelizedName + " (reserved word) cannot be used as model name. Renamed to " + camelizedName);
        }

        // model name starts with number
        else if (name.matches("^\\d.*")) {
            // e.g. 200Response => Model200Response (after camelize)
            camelizedName = "Model" + camelizedName;
            LOGGER.warn(name + " (model name starts with number) cannot be used as model name. Renamed to "
                    + camelizedName);
        }

        return camelizedName;

    }

    @Override
    public String toParamName(String name) {
        // should be the same as variable name (stolen from RubyClientCodegen)
        return toVarName(name);
    }

    @Override
    public String toVarName(String name) {
        if (name.matches("^[-+][0-9]+$")) {
            name = name.replaceAll("-", "MINUS_");
            name = name.replaceAll("\\+", "PLUS_");
        }
        String sanitizedName = sanitizeName(name);
        // for reserved word or word starting with number, append _
        if (isReservedWord(sanitizedName) || sanitizedName.matches("^\\d.*")) {
            sanitizedName = escapeReservedWord(sanitizedName);
        }

        return underscore(sanitizedName);
    }

    @Override
    public String toOperationId(String operationId) {
        // method name cannot use reserved keyword, e.g. return
        if (isReservedWord(operationId)) {
            LOGGER.warn(operationId + " (reserved word) cannot be used as method name. Renamed to "
                    + camelize(sanitizeName("call_" + operationId)));
            operationId = "call_" + operationId;
        }

        return underscore(camelize(operationId));
    }

    @Override
    public String toModelFilename(String name) {
        if (!StringUtils.isEmpty(modelNamePrefix)) {
            name = modelNamePrefix + "_" + name;
        }

        if (!StringUtils.isEmpty(modelNameSuffix)) {
            name = name + "_" + modelNameSuffix;
        }

        name = sanitizeName(name);

        // model name cannot use reserved keyword, e.g. return
        if (isReservedWord(name)) {
            LOGGER.warn(
                    name + " (reserved word) cannot be used as model name. Renamed to " + camelize("model_" + name));
            name = "model_" + name; // e.g. return => ModelReturn (after camelize)
        }

        return underscore(name);
    }

    @Override
    public String toEnumName(CodegenProperty property) {
        return sanitizeName(camelize(property.name)) + "Enum";
    }

    @Override
    public String toEnumVarName(String value, String datatype) {
        String var = null;
        if (value.length() == 0) {
            var = "EMPTY";
        }

        // for symbol, e.g. $, #
        else if (getSymbolName(value) != null) {
            var = getSymbolName(value).toUpperCase();
        }

        // number
        else if ("Integer".equals(datatype) || "Long".equals(datatype) || "Float".equals(datatype)
                || "Double".equals(datatype)) {
            String varName = "NUMBER_" + value;
            varName = varName.replaceAll("-", "MINUS_");
            varName = varName.replaceAll("\\+", "PLUS_");
            varName = varName.replaceAll("\\.", "_DOT_");
            var = varName;
        }

        // string
        var = value.replaceAll("\\W+", "_").toUpperCase();
        if (var.matches("\\d.*")) {
            var = "_" + var;
        } else {
            var = sanitizeName(var);
        }
        return var;
    }

    @Override
    public String toEnumValue(String value, String datatype) {
        if ("Integer".equals(datatype) || "Long".equals(datatype) || "Float".equals(datatype)
                || "Double".equals(datatype)) {
            return value;
        } else {
            return "\"" + escapeText(value) + "\"";
        }
    }

    /**
     * Configures the type of generator.
     *
     * @return the CodegenType for this generator
     * @see CodegenType
     */
    @Override
    public CodegenType getTag() {
        return CodegenType.CLIENT;
    }

    /**
     * Configures a friendly name for the generator. This will be used by the
     * generator to select the library with the -l flag.
     *
     * @return the friendly name for the generator
     */
    @Override
    public String getName() {
        return "rust";
    }

    /**
     * Returns human-friendly help for the generator. Provide the consumer with help
     * tips, parameters here
     *
     * @return A string value for the help message
     */
    @Override
    public String getHelp() {
        return "Generates a Rust client library (beta) using the swagger-codegen project.";
    }

    @Override
    public String toApiFilename(String name) {
        // replace - with _ e.g. created-at => created_at
        name = name.replaceAll("-", "_");

        // e.g. PetApi.go => pet_api.go
        return underscore(name);
    }

    @Override
    public String escapeQuotationMark(String input) {
        // remove " to avoid code injection
        return input.replace("\"", "");
    }

    @Override
    public String escapeUnsafeCharacters(String input) {
        return input.replace("*/", "*_/").replace("/*", "/_*");
    }

    boolean isMimetypeXml(String mimetype) {
        return mimetype.toLowerCase().startsWith("application/xml");
    }

    boolean isMimetypePlainText(String mimetype) {
        return mimetype.toLowerCase().startsWith("text/plain");
    }

    boolean isMimetypeWwwFormUrlEncoded(String mimetype) {
        return mimetype.toLowerCase().startsWith("application/x-www-form-urlencoded");
    }

    @Override
    public CodegenOperation fromOperation(String path, String httpMethod, Operation operation,
            Map<String, Schema> definitions, OpenAPI openapi) {

        String operationId = getOrGenerateOperationId(operation, path, httpMethod);
        RequestBody body = operation.getRequestBody();
        if (body != null) {
            if (StringUtils.isNotBlank(body.get$ref())) {
                String bodyName = OpenAPIUtil.getSimpleRef(body.get$ref());
                body = openAPI.getComponents().getRequestBodies().get(bodyName);
            }

            body.addExtension("x-codegen-operation-name", toSanitizedOperationBodyName(httpMethod, operationId));

        }

        if (operation.getResponses() != null && !operation.getResponses().isEmpty()) {
            ApiResponse methodResponse = findMethodResponse(operation.getResponses());
            for (String key : operation.getResponses().keySet()) {
                ApiResponse response = operation.getResponses().get(key);
                response.addExtension("x-codegen-operation-name",
                        toSanitizedOperationBodyName(httpMethod, operationId));
            }
        }

        CodegenOperation op = super.fromOperation(path, httpMethod, operation, definitions, openapi);

        // The Rust code will need to contain a series of regular expressions.
        // For performance, we'll construct these at start-of-day and re-use
        // them. That means we need labels for them.
        //
        // Construct a Rust constant (uppercase) token name, and ensure it's
        // unique using a numeric tie-breaker if required.
        String basePathId = sanitizeName(
                op.path.replace("/", "_").replace("{", "").replace("}", "").replaceAll("^_", "")).toUpperCase();
        String pathId = basePathId;
        int pathIdTiebreaker = 2;
        boolean found = false;
        while (pathSetMap.containsKey(pathId)) {
            Map<String, String> pathSetEntry = pathSetMap.get(pathId);
            if (pathSetEntry.get("path").equals(op.path)) {
                found = true;
                break;
            }
            pathId = basePathId + pathIdTiebreaker;
            pathIdTiebreaker++;
        }

        // Save off the regular expression and path details in the
        // "pathSetMap", which we'll add to the source document that will be
        // processed by the templates.
        if (!found) {
            Map<String, String> pathSetEntry = new HashMap<String, String>();
            pathSetEntry.put("path", op.path);
            pathSetEntry.put("PATH_ID", pathId);
            if (!op.pathParams.isEmpty()) {
                pathSetEntry.put("hasPathParams", "true");
            }
            // Don't prefix with '^' so that the templates can put the
            // basePath on the front.
            pathSetEntry.put("pathRegEx", op.path.replace("{", "(?P<").replace("}", ">[^/?#]*)") + "$");
            pathSetMap.put(pathId, pathSetEntry);
        }

        op.vendorExtensions.put("operation_id", underscore(op.operationId));
        op.vendorExtensions.put("uppercase_operation_id", underscore(op.operationId).toUpperCase());
        op.vendorExtensions.put("path", op.path.replace("{", ":").replace("}", ""));
        op.vendorExtensions.put("PATH_ID", pathId);
        op.vendorExtensions.put("hasPathParams", !op.pathParams.isEmpty());
        op.vendorExtensions.put("HttpMethod",
                Character.toUpperCase(op.httpMethod.charAt(0)) + op.httpMethod.substring(1).toLowerCase());

        Set<String> consumes = new HashSet<String>();
        if (super.getConsumesInfo(operation) != null) {
            if (super.getConsumesInfo(operation).size() > 0) {
                // use consumes defined in the operation
                consumes = super.getConsumesInfo(operation);
            }
        }

        boolean consumesPlainText = false;
        boolean consumesXml = false;
        // if "consumes" is defined (per operation or using global definition)
        if (consumes != null && !consumes.isEmpty()) {
            List<Map<String, String>> c = new ArrayList<Map<String, String>>();
            for (String mimeType : consumes) {
                Map<String, String> mediaType = new HashMap<String, String>();

                if (isMimetypeXml(mimeType)) {
                    additionalProperties.put("usesXml", true);
                    consumesXml = true;
                } else if (isMimetypePlainText(mimeType)) {
                    consumesPlainText = true;
                } else if (isMimetypeWwwFormUrlEncoded(mimeType)) {
                    additionalProperties.put("usesUrlEncodedForm", true);
                }

                mediaType.put("mediaType", mimeType);
                c.add(mediaType);
            }
            op.consumes = c;
            op.getVendorExtensions().put(CodegenConstants.HAS_CONSUMES_EXT_NAME, Boolean.TRUE);
        }

        Set<String> produces = new HashSet<String>();
        if (super.getProducesInfo(operation) != null) {
            if (super.getProducesInfo(operation).size() > 0) {
                // use produces defined in the operation
                produces = super.getProducesInfo(operation);
            }
        }

        boolean producesXml = false;
        boolean producesPlainText = false;
        if (produces != null && !produces.isEmpty()) {
            List<Map<String, String>> c = new ArrayList<Map<String, String>>();
            for (String mimeType : produces) {
                Map<String, String> mediaType = new HashMap<String, String>();

                if (isMimetypeXml(mimeType)) {
                    additionalProperties.put("usesXml", true);
                    producesXml = true;
                } else if (isMimetypePlainText(mimeType)) {
                    producesPlainText = true;
                }

                mediaType.put("mediaType", mimeType);
                c.add(mediaType);
            }
            op.produces = c;
            op.getVendorExtensions().put(CodegenConstants.HAS_PRODUCES_EXT_NAME, Boolean.TRUE);
        }

        if (op.bodyParam != null) {
            for (String key : definitions.keySet()) {
                op.bodyParam.vendorExtensions.put("model_key", key);
            }

            // Default to consuming json
            op.bodyParam.vendorExtensions.put("uppercase_operation_id", underscore(op.operationId).toUpperCase());
            if (consumesXml) {
                op.bodyParam.vendorExtensions.put("consumesXml", true);
            } else if (consumesPlainText) {
                op.bodyParam.vendorExtensions.put("consumesPlainText", true);
            } else {
                op.bodyParam.vendorExtensions.put("consumesJson", true);
            }

        }
        for (CodegenParameter param : op.bodyParams) {

            param.vendorExtensions.put("uppercase_operation_id", underscore(op.operationId).toUpperCase());

            // Default to producing json if nothing else is specified
            if (consumesXml) {
                param.vendorExtensions.put("consumesXml", true);
            } else if (consumesPlainText) {
                param.vendorExtensions.put("consumesPlainText", true);
            } else {
                param.vendorExtensions.put("consumesJson", true);
            }
        }
        for (CodegenParameter param : op.headerParams) {
            // If a header uses UUIDs, we need to import the UUID package.
            if (param.dataType.equals("uuid::Uuid")) {
                additionalProperties.put("apiUsesUuid", true);
            }

            // Give header params a name in camel case. CodegenParameters don't have a
            // nameInCamelCase property.
            param.vendorExtensions.put("typeName", toModelName(param.baseName));
        }

        for (CodegenParameter param : op.queryParams) {
            if (param.getDataType() != null && param.getDataType().equals("Bool")) {
                param.dataType = "bool";
                param.vendorExtensions.put(CodegenConstants.IS_BOOLEAN_EXT_NAME, "true");
            }
        }
        for (CodegenResponse rsp : op.responses) {
            if (rsp.getMessage() != null) {
                String[] words = rsp.getMessage().split("[^A-Za-z ]");
                String responseId;
                if (rsp.vendorExtensions.containsKey("x-responseId")) {
                    responseId = (String) rsp.vendorExtensions.get("x-responseId");
                } else if (words.length != 0) {
                    responseId = camelize(words[0].replace(" ", "_"));
                } else {
                    responseId = "Status" + rsp.code;
                }
                rsp.vendorExtensions.put("x-responseId", responseId);
                rsp.vendorExtensions.put("x-uppercaseResponseId", underscore(responseId).toUpperCase());
            }
            rsp.vendorExtensions.put("uppercase_operation_id", underscore(op.operationId).toUpperCase());
            if (rsp.dataType != null) {
                rsp.vendorExtensions.put("uppercase_data_type", (rsp.dataType.replace("models::", "")).toUpperCase());

                // Default to producing json if nothing else is specified
                if (producesXml) {
                    rsp.vendorExtensions.put("producesXml", true);
                } else if (producesPlainText) {
                    rsp.vendorExtensions.put("producesPlainText", true);
                } else {
                    rsp.vendorExtensions.put("producesJson", true);
                }
            }
            for (CodegenProperty header : rsp.headers) {
                if (header.datatype.equals("uuid::Uuid")) {
                    additionalProperties.put("apiUsesUuid", true);
                }
                header.nameInCamelCase = toModelName(header.baseName);
            }
        }
        for (CodegenProperty header : op.responseHeaders) {
            if (header.datatype.equals("uuid::Uuid")) {
                additionalProperties.put("apiUsesUuid", true);
            }
            header.nameInCamelCase = toModelName(header.baseName);
        }

        return op;
    }

    private String toSanitizedOperationBodyName(String httpMethod, String operationId) {
        return httpMethod + camelize(operationId);
    }

    @Override
    public boolean isDataTypeFile(final String dataType) {
        return dataType != null && dataType.equals(typeMapping.get("File").toString());
    }

    @Override
    public String getTypeDeclaration(Schema p) {
        if (p instanceof ArraySchema) {
            ArraySchema ap = (ArraySchema) p;
            Schema inner = ap.getItems();
            String innerType = getTypeDeclaration(inner);
            StringBuilder typeDeclaration = new StringBuilder(typeMapping.get("array")).append("<");
            typeDeclaration.append(innerType).append(">");
            return typeDeclaration.toString();
        } else if (p instanceof MapSchema) {
            MapSchema mp = (MapSchema) p;
            Object inner = mp.getAdditionalProperties();
            if (inner instanceof Schema) {
                String schemaType = getSchemaType((Schema) inner);
                String innerType;
                if (!typeMapping.containsKey(schemaType)) {
                    innerType = toModelName(getTypeDeclaration((Schema) inner));
                } else {
                    innerType = getTypeDeclaration((Schema) inner);
                }
                StringBuilder typeDeclaration = new StringBuilder(typeMapping.get("map")).append("<")
                        .append(typeMapping.get("string")).append(", ");
                typeDeclaration.append(innerType).append(">");
                return typeDeclaration.toString();
            }
        } else if (p instanceof ComposedSchema) {
            String datatype;
            try {
                ComposedSchema r = (ComposedSchema) p;
                datatype = r.get$ref();
                if (r.getAllOf() != null && r.getAllOf().size() == 1) {
                    datatype = getTypeDeclaration(r.getAllOf().get(0));
                } else if (r.getAllOf() != null) {
                    LOGGER.warn("Found more than one type in composed schema: " + p + ".");
                }
            } catch (Exception e) {
                LOGGER.warn("Error obtaining the datatype from RefProperty:" + p + ". Datatype default to Object");
                datatype = super.getTypeDeclaration(p);
                LOGGER.error(e.getMessage(), e);
            }
            return datatype;
        } else if (p instanceof FileSchema) {
            return typeMapping.get("File").toString();
        }
        return super.getTypeDeclaration(p);
    }

    @Override
    public CodegenProperty fromProperty(String name, Schema p) {
        CodegenProperty property = super.fromProperty(name, p);
        if (p instanceof ArraySchema && !property.datatype.startsWith("#")) {
            ArraySchema ap = (ArraySchema) p;
            Schema inner = ap.getItems();
            String innerType = getSchemaType(inner);
            if (innerType != null) {
                if (innerType.startsWith("#") || innerType.equals("object")) {
                    property.getItems().setDatatype("Value");
                    property.getItems().setComplexType("Value");
                    property.setComplexType("Value");
                } else if (inner instanceof IntegerSchema) {
                    property.getItems().setDatatype("i32");
                    property.getItems().setComplexType("i32");
                    property.setComplexType("i32");
                } else {
                    property.getItems().setDatatype(toModelName(innerType));
                    property.getItems().setComplexType(toModelName(innerType));
                    property.setComplexType(toModelName(innerType));
                    property.setDatatype("Vec<" + toModelName(innerType) + ">");
                }
            }

        } else if (p instanceof MapSchema) {
            MapSchema mp = (MapSchema) p;
            Object inner = mp.getAdditionalProperties();
            if (inner instanceof Schema) {
                String schemaType = getSchemaType((Schema) inner);
                String innerType;
                if (!typeMapping.containsKey(schemaType)) {
                    innerType = toModelName(getTypeDeclaration((Schema) inner));
                } else {
                    innerType = getTypeDeclaration((Schema) inner);
                }

                property.getItems().setDatatype(innerType);
                property.getItems().setComplexType(innerType);
                property.setComplexType(innerType);
            }
        }

        if (getSchemaType(p) != null && getSchemaType(p).contains(" ")) {
            property.datatype = toModelName(getSchemaType(p));
        }

        // All unknown datatypes mapped to Value
        if (property.getDatatype() == null) {
            property.setDatatype("Value");
        }
        if (property.getDatatype().equals("datetime")) {
            property.setDatatype("DateTime<Utc>");
        }
        if (!(p instanceof ArraySchema) && getSchemaType(p) != null && getSchemaType(p).equals("array")) {
            // fallback array type
            property.setDatatype("Vec<Value>");
        }
        return property;
    }

    @Override
    public String toInstantiationType(Schema p) {
        if (p instanceof ArraySchema) {
            ArraySchema ap = (ArraySchema) p;
            Schema inner = ap.getItems();
            return instantiationTypes.get("array") + "<" + getSchemaType(inner) + ">";
        } else if (p instanceof MapSchema) {
            MapSchema mp = (MapSchema) p;
            if (mp.getAdditionalProperties() != null && mp.getAdditionalProperties() instanceof Schema) {
                return instantiationTypes.get("map") + "<" + typeMapping.get("string") + ", "
                        + getSchemaType((Schema) mp.getAdditionalProperties()) + ">";
            }
        } else {
            return null;
        }
        return super.toInstantiationType(p);
    }

    @Override
    public CodegenModel fromModel(String name, Schema schema, Map<String, Schema> allDefinitions) {
        CodegenModel mdl = super.fromModel(name, schema, allDefinitions);
        mdl.vendorExtensions.put("upperCaseName", name.toUpperCase());

        return mdl;
    }

    @Override
    public Map<String, Object> postProcessAllModels(Map<String, Object> objs) {
        Map<String, Object> newObjs = super.postProcessAllModels(objs);

        // Index all CodegenModels by model name.
        HashMap<String, CodegenModel> allModels = new HashMap<String, CodegenModel>();
        for (Map.Entry<String, Object> entry : objs.entrySet()) {
            String modelName = toModelName(entry.getKey());
            Map<String, Object> inner = (Map<String, Object>) entry.getValue();
            List<Map<String, Object>> models = (List<Map<String, Object>>) inner.get("models");
            for (Map<String, Object> mo : models) {
                CodegenModel cm = (CodegenModel) mo.get("model");
                allModels.put(modelName, cm);
            }
        }

        return newObjs;
    }

    @Override
    public String toDefaultValue(Schema p) {
        if (p instanceof StringSchema) {
            StringSchema dp = (StringSchema) p;
            if (dp.getDefault() != null) {
                return "\"" + dp.getDefault() + "\".to_string()";
            }
        } else if (p instanceof BooleanSchema) {
            BooleanSchema dp = (BooleanSchema) p;
            if (dp.getDefault() != null) {
                if (dp.getDefault().toString().equalsIgnoreCase("false"))
                    return "false";
                else
                    return "true";
            }
        } else if (p instanceof NumberSchema) {
            if (SchemaTypeUtil.FLOAT_FORMAT.equals(p.getFormat())) {
                if (p.getDefault() != null) {
                    return p.getDefault().toString();
                }
            } else if (SchemaTypeUtil.DOUBLE_FORMAT.equals(p.getFormat())) {
                if (p.getDefault() != null) {
                    return p.getDefault().toString();
                }
            }
        } else if (p instanceof IntegerSchema) {
            if (SchemaTypeUtil.INTEGER64_FORMAT.equals(p.getFormat())) {
                return p.getDefault().toString();
            } else if (SchemaTypeUtil.INTEGER32_FORMAT.equals(p.getFormat())) {
                return p.getDefault().toString();
            }
        }

        return null;
    }

    @Override
    public void postProcessModelProperty(CodegenModel model, CodegenProperty property) {
        super.postProcessModelProperty(model, property);

        if (property.datatype.startsWith("#")) {
            property.datatype = "Value";

        } else if (!languageSpecificPrimitives.contains(property.datatype)) {
            // If we use a more qualified model name, then only camelize the actual type,
            // not the qualifier.
            if (property.datatype.contains(":")) {
                int position = property.datatype.lastIndexOf(":");
                property.datatype = property.datatype.substring(0, position)
                        + camelize(property.datatype.substring(position));
            } else {
                property.datatype = camelize(property.datatype, false);
            }
        }

        if ("integer".equals(property.baseType)) {
            // custom integer formats (legacy)
            if ("uint32".equals(property.dataFormat)) {
                property.datatype = "u32";
            } else if ("uint64".equals(property.dataFormat)) {
                property.datatype = "u64";

            } else {
                // match int type to schema constraints
                Long inclusiveMinimum = property.minimum != null ? Long.parseLong(property.minimum) : null;
                if (inclusiveMinimum != null && property.exclusiveMinimum) {
                    inclusiveMinimum++;
                }

                // a signed int is required unless a minimum greater than zero is set
                boolean unsigned = inclusiveMinimum != null && inclusiveMinimum >= 0;

                Long inclusiveMaximum = property.maximum != null ? Long.parseLong(property.maximum) : null;
                if (inclusiveMaximum != null && property.exclusiveMaximum) {
                    inclusiveMaximum--;
                }

                if ("int32".equals(property.dataFormat)) {
                    property.datatype = unsigned ? "u32" : "i32";
                } else if ("int64".equals(property.dataFormat)) {
                    property.datatype = unsigned ? "u64" : "i64";
                } else {
                    property.datatype = matchingIntType(unsigned, inclusiveMinimum, inclusiveMaximum);
                }
            }
        }

        property.name = underscore(property.name);

        if (!property.required) {
            property.defaultValue = (property.defaultValue != null) ? "Some(" + property.defaultValue + ")" : "None";
        }
    }

    static long requiredBits(Long bound, boolean unsigned) {
        if (bound == null)
            return 0;

        if (unsigned) {
            if (bound < 0) {
                throw new RuntimeException("Unsigned bound is negative: " + bound);
            }
            return 65 - Long.numberOfLeadingZeros(bound >> 1);
        }

        return 65 - Long.numberOfLeadingZeros(
                // signed bounds go from (-n) to (n - 1), i.e. i8 goes from -128 to 127
                bound < 0 ? Math.abs(bound) - 1 : bound);
    }

    static String matchingIntType(boolean unsigned, Long inclusiveMin, Long inclusiveMax) {
        long requiredMinBits = requiredBits(inclusiveMin, unsigned);
        long requiredMaxBits = requiredBits(inclusiveMax, unsigned);
        long requiredBits = Math.max(requiredMinBits, requiredMaxBits);

        if (requiredMaxBits == 0 && requiredMinBits <= 16) {
            // rust 'size' types are arch-specific and thus somewhat loose
            // so they are used when no format or maximum are specified
            // and as long as minimum stays within plausible smallest ptr size (16 bits)
            // this way all rust types are obtainable without defining custom formats
            // this behavior (default int size) could also follow a generator flag
            return unsigned ? "usize" : "isize";

        } else if (requiredBits <= 8) {
            return unsigned ? "u8" : "i8";

        } else if (requiredBits <= 16) {
            return unsigned ? "u16" : "i16";

        } else if (requiredBits <= 32) {
            return unsigned ? "u32" : "i32";
        }
        return unsigned ? "u64" : "i64";
    }

    @Override
    public Map<String, Object> postProcessModels(Map<String, Object> objs) {
        return super.postProcessModelsEnum(objs);
    }

    @Override
    public String getDefaultTemplateDir() {
        return "rust";
    }

    // @Override
    public CodegenResponse fromResponse(String responseCode, ApiResponse response) {
        CodegenResponse res = super.fromResponse(responseCode, response);
        String dataType = res.getDataType();
        if (dataType != null) {
            Schema p = (Schema) res.getSchema();
            if (p instanceof ArraySchema) {
                ArraySchema ap = (ArraySchema) p;
                Schema inner = ap.getItems();
                if (inner instanceof Schema) {
                    String schemaType = getSchemaType((Schema) inner);
                    String innerType;
                    if (!typeMapping.containsKey(schemaType)) {
                        innerType = toModelName(getTypeDeclaration((Schema) inner));
                    } else {
                        innerType = getTypeDeclaration((Schema) inner);
                    }
                    StringBuilder typeDeclaration = new StringBuilder(typeMapping.get("array")).append("<");
                    typeDeclaration.append(innerType).append(">");
                    res.dataType = typeDeclaration.toString();
                }
            } else if (p instanceof MapSchema) {
                MapSchema mp = (MapSchema) p;
                Object inner = mp.getAdditionalProperties();
                if (inner instanceof Schema) {
                    String schemaType = getSchemaType((Schema) inner);
                    String innerType;
                    if (!typeMapping.containsKey(schemaType)) {
                        innerType = toModelName(getTypeDeclaration((Schema) inner));
                    } else {
                        innerType = getTypeDeclaration((Schema) inner);
                    }
                    StringBuilder typeDeclaration = new StringBuilder(typeMapping.get("map")).append("<")
                            .append(typeMapping.get("string")).append(", ");
                    typeDeclaration.append(innerType).append(">");
                    res.dataType = typeDeclaration.toString();
                }
            } else {
                res.dataType = camelize(dataType);
            }
        }
        return res;
    }

    @Override
    public Map<String, Object> postProcessOperations(Map<String, Object> objs) {
        objs = super.postProcessOperations(objs);
        Map<String, Object> operations = (Map<String, Object>) objs.get("operations");
        if (operations != null) {
            List<CodegenOperation> ops = (List<CodegenOperation>) operations.get("operation");
            for (final CodegenOperation operation : ops) {
                String path = operation.getPath();
                if (path != null) {
                    operation.path = path.replaceAll("\\{[^\\}]*\\}", "{}");
                }
            }
        }
        return objs;
    }

    @Override
    public CodegenParameter fromParameter(Parameter parameter, Set<String> imports) {
        CodegenParameter codegenParameter = super.fromParameter(parameter, imports);
        Schema parameterSchema = parameter.getSchema();
        if (parameterSchema == null) {
            parameterSchema = getSchemaFromParameter(parameter);
        }
        if (parameterSchema != null) {
            String collectionFormat = null;
            if (!(parameterSchema instanceof ArraySchema) && !(parameterSchema instanceof MapSchema)
                    && !(parameterSchema instanceof FileSchema) && !(parameterSchema instanceof BinarySchema)
                    && !(parameterSchema instanceof IntegerSchema)) {
                codegenParameter.dataType = camelize(codegenParameter.dataType);
            }
        }
        return codegenParameter;
    }
}
