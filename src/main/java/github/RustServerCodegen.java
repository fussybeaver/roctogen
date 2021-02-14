package github;

import io.swagger.codegen.v3.CliOption;
import io.swagger.codegen.v3.CodegenArgument;
import io.swagger.codegen.v3.CodegenConstants;
import io.swagger.codegen.v3.CodegenModel;
import io.swagger.codegen.v3.CodegenOperation;
import io.swagger.codegen.v3.CodegenParameter;
import io.swagger.codegen.v3.CodegenProperty;
import io.swagger.codegen.v3.CodegenResponse;
import io.swagger.codegen.v3.CodegenType;
import io.swagger.codegen.v3.generators.DefaultCodegenConfig;
import io.swagger.codegen.v3.SupportingFile;
import io.swagger.codegen.v3.utils.URLPathUtil;
import io.swagger.codegen.v3.generators.util.OpenAPIUtil;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.PathItem;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.BooleanSchema;
import io.swagger.v3.oas.models.media.ComposedSchema;
import io.swagger.v3.oas.models.media.IntegerSchema;
import io.swagger.v3.oas.models.media.FileSchema;
import io.swagger.v3.oas.models.media.MapSchema;
import io.swagger.v3.oas.models.media.NumberSchema;
import io.swagger.v3.oas.models.media.ObjectSchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.StringSchema;
import io.swagger.v3.oas.models.responses.ApiResponse;
import io.swagger.v3.oas.models.parameters.Parameter;
import io.swagger.v3.oas.models.parameters.RequestBody;
import io.swagger.v3.parser.util.SchemaTypeUtil;
import org.apache.commons.lang3.BooleanUtils;
import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.util.Arrays;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;
import java.util.ListIterator;
import java.util.Map;
import java.util.Set;
import java.util.regex.Pattern;
import java.net.URL;

import static io.swagger.codegen.v3.generators.handlebars.ExtensionHelper.getBooleanValue;

public class RustServerCodegen extends DefaultCodegenConfig {
    private static final Logger LOGGER = LoggerFactory.getLogger(RustServerCodegen.class);

    private HashMap<String, String> modelXmlNames = new HashMap<String, String>();

    private static final String NO_FORMAT = "%%NO_FORMAT";

    protected String apiVersion = "1.0.0";
    protected String projectName = "swagger-client";
    protected String apiPath = "rust-client";
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
        modelTemplateFiles.clear();

        /*
         * Api classes. You can write classes for each Api file with the
         * apiTemplateFiles map. as with models, add multiple entries with different
         * extensions for multiple files per class
         */
        apiTemplateFiles.clear();

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
        // typeMapping.put("number", "f64");
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
        supportingFiles.add(new SupportingFile("Cargo.mustache", "", "Cargo.toml"));
        supportingFiles.add(new SupportingFile("cargo-config", ".cargo", "config"));
        supportingFiles.add(new SupportingFile("gitignore", "", ".gitignore"));
        supportingFiles.add(new SupportingFile("lib.mustache", "src", "lib.rs"));
        supportingFiles.add(new SupportingFile("models.mustache", "src", "models.rs"));
        supportingFiles.add(new SupportingFile("api.mustache", "src", "api.rs"));
        supportingFiles.add(new SupportingFile("client.mustache", "src", "client.rs"));

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
        return underscore(name);
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

        return camelize(operationId);
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
        for (CodegenParameter param : op.allParams) {
            processParam(param, op);
        }

        Set<String> consumes = new HashSet<String>();
        if (super.getConsumesInfo(operation) != null) {
            if (super.getConsumesInfo(operation).size() > 0) {
                // use consumes defined in the operation
                consumes = super.getConsumesInfo(operation);
            }
            // } else if (openapi != null && openapi.getConsumes() != null &&
            // openapi.getConsumes().size() > 0) {
            // // use consumes defined globally
            // consumes = openapi.getConsumes();
            // LOGGER.debug("No consumes defined in operation. Using global consumes (" +
            // openapi.getConsumes() + ") for " + op.operationId);
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
            // } else if (openapi != null && openapi.getProduces() != null &&
            // openapi.getProduces().size() > 0) {
            // // use produces defined globally
            // produces = openapi.getProduces();
            // LOGGER.debug("No produces defined in operation. Using global produces (" +
            // openapi.getProduces() + ") for " + op.operationId);
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
            if (paramHasXmlNamespace(op.bodyParam, definitions)) {
                op.bodyParam.vendorExtensions.put("has_namespace", "true");
            }
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
            processParam(param, op);

            if (paramHasXmlNamespace(param, definitions)) {
                param.vendorExtensions.put("has_namespace", "true");
            }

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
            processParam(param, op);

            // Give header params a name in camel case. CodegenParameters don't have a
            // nameInCamelCase property.
            param.vendorExtensions.put("typeName", toModelName(param.baseName));
        }
        for (CodegenParameter param : op.formParams) {
            processParam(param, op);
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

                // Check whether we're returning an object with a defined XML namespace.
                // Schema property = rsp.schema;
                // if ((property != null) && (property instanceof ComposedSchema)){

                // ComposedSchema refProperty = (ComposedSchema) property;
                // String refName = refProperty.get$ref();
                // if (refName.indexOf("#/definitions/") == 0) {
                // refName = refName.substring("#/definitions/".length());
                // }

                // Model model = definitions.get(refName);

                // if ((model != null) && (model instanceof ModelImpl)) {
                // Xml xml = ((ModelImpl) model).getXml();
                // if ((xml != null) && (xml.getNamespace() != null)){
                // rsp.vendorExtensions.put("has_namespace", "true");
                // }
                // }
                // }
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

    // @Override
    // public CodegenParameter fromRequestBody(RequestBody body, String name, Schema
    // schema, Map<String, Schema> schemas,
    // Set<String> imports) {
    // CodegenParameter param = super.fromRequestBody(body, name, schema, schemas,
    // imports);

    // if (body.getExtensions() != null) {
    // Object operationName = body.getExtensions().get("x-codegen-operation-name");
    // if (operationName != null && !operationName.toString().isEmpty() && name !=
    // null) {
    // Map<String, Object> extensions = param.getVendorExtensions();
    // extensions.put("x-codegen-operation-name", operationName);
    // }
    // }

    // return param;
    // }

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
            if (inner instanceof ComposedSchema) {
                // typeDeclaration.append("models::");
            }
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
                if (inner instanceof ComposedSchema) {
                    // typeDeclaration.append("models::");
                }
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
                    // datatype = toModelName(r.getAllOf().get(0));
                } else if (r.getAllOf() != null) {
                    LOGGER.warn("Found more than one type in composed schema: " + p + ".");
                }
            } catch (Exception e) {
                LOGGER.warn("Error obtaining the datatype from RefProperty:" + p + ". Datatype default to Object");
                datatype = "Object";
                LOGGER.error(e.getMessage(), e);
            }
            return datatype;
        } else if (p instanceof FileSchema) {
            return typeMapping.get("File").toString();
        }
        return super.getTypeDeclaration(p);
    }

    @Override
    public CodegenParameter fromParameter(Parameter param, Set<String> imports) {
        CodegenParameter parameter = super.fromParameter(param, imports);
        // BodyParameter bp = (BodyParameter) param;
        // Model model = bp.getSchema();
        // if (model instanceof RefModel) {
        // String name = ((RefModel) model).getSimpleRef();
        // name = toModelName(name);
        //// We need to be able to look up the model in the model definitions later.
        // parameter.vendorExtensions.put("uppercase_data_type", name.toUpperCase());

        //// name = "models::" + getTypeDeclaration(name);
        //// parameter.baseType = name;
        //// parameter.dataType = name;

        // String refName = ((RefModel) model).get$ref();
        // if (refName.indexOf("#/definitions/") == 0) {
        // refName = refName.substring("#/definitions/".length());
        // }
        // parameter.vendorExtensions.put("refName", refName);

        // } else if (model instanceof ModelImpl) {
        // parameter.vendorExtensions.put("refName", ((ModelImpl) model).getName());
        // }
        // }
        return parameter;
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

        // TODO: property.datatype.startsWith("#/paths")

        // TODO: move below to subclass
        // All unknown datatypes mapped to Value
        if (property.getDatatype() == null) {
            property.setDatatype("Value");
        }
        if (property.getDatatype().equals("datetime")) {
            property.setDatatype("DateTime<Utc>");
        }
        if (getSchemaType(p) != null && getSchemaType(p).equals("array")) {
            property.setComplexType("Value");
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
        // if (model instanceof ModelImpl) {
        // ModelImpl modelImpl = (ModelImpl) model;
        // mdl.dataType = typeMapping.get(modelImpl.getType());
        // }
        // if (model instanceof ArraySchema) {
        // ArraySchema am = (ArraySchema) model;
        // if ((am.getItems() != null) && (am.getItems().getXml() != null)) {

        // // If this model's items require wrapping in xml, squirrel
        // // away the xml name so we can insert it into the relevant model fields.
        // // String xmlName = am.getItems().getXml().getName();
        // // if (xmlName != null) {
        // // mdl.vendorExtensions.put("itemXmlName", xmlName);
        // // modelXmlNames.put("models::" + mdl.classname, xmlName);
        // // }
        // }
        // mdl.arrayModelType = toModelName(mdl.arrayModelType);
        // }

        // if (mdl.xmlNamespace != null) {
        // additionalProperties.put("usesXmlNamespaces", true);
        // }

        // if (schema instanceof ComposedSchema) {
        // final ComposedSchema composed = (ComposedSchema) schema;
        // Map<String, Schema> allProperties;
        // List<String> allRequired;
        // allProperties = null;
        // allRequired = null;
        // final String parentName = getParentName(composed);
        // final Schema parent = StringUtils.isBlank(parentName) ? null :
        // allDefinitions.get(parentName);
        // final List<Schema> allOf = composed.getAllOf();
        // // interfaces (intermediate models)
        // LOGGER.info(" ::: " + name);
        // if (allOf != null && !allOf.isEmpty()) {

        // LOGGER.info(" ::: size " + allOf.size());
        // for (int i = 0; i < allOf.size(); i++) {
        // if (i == 0 && !copyFistAllOfProperties) {
        // continue;
        // }
        // Schema interfaceSchema = allOf.get(i);
        // LOGGER.info(" ::: interfaceSchema " + interfaceSchema.get$ref());
        // if (StringUtils.isBlank(interfaceSchema.get$ref())) {
        // continue;
        // }
        // Schema refSchema = null;
        // String ref = OpenAPIUtil.getSimpleRef(interfaceSchema.get$ref());
        // if (allDefinitions != null) {
        // refSchema = allDefinitions.get(ref);
        // }
        // final String modelName = toModelName(ref);
        // LOGGER.info(" ::: ref " + ref);
        // // addImport(codegenModel, modelName);
        // LOGGER.info(" ::: addImport " + modelName);
        // LOGGER.info(" ::: allDefinitions " + (allDefinitions != null));
        // LOGGER.info(" ::: refSchema " + (refSchema != null));
        // if (allDefinitions != null && refSchema != null) {
        // if (!supportsMixins) {
        // LOGGER.info(" ::: addProperties ");
        // // addProperties(properties, required, refSchema, allDefinitions);
        // }
        // if (supportsInheritance) {
        // // addProperties(allProperties, allRequired, refSchema, allDefinitions);
        // }
        // }
        // }
        // }
        // }

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

        // for (Map.Entry<String, CodegenModel> entry : allModels.entrySet()) {
        // String modelName = entry.getKey();
        // CodegenModel model = entry.getValue();

        // for (CodegenProperty prop : model.vars) {
        // String xmlName = modelXmlNames.get(prop.datatype);
        // if (xmlName != null) {
        // prop.vendorExtensions.put("itemXmlName", xmlName);
        // }
        // }
        // }

        return newObjs;
    }

    @Override
    public Map<String, Object> postProcessSupportingFileData(Map<String, Object> objs) {
        // Swagger swagger = (Swagger)objs.get("swagger");
        // if(swagger != null) {
        // try {
        // objs.put("swagger-yaml", Yaml.mapper().writeValueAsString(swagger));
        // } catch (JsonProcessingException e) {
        // LOGGER.error(e.getMessage(), e);
        // }
        // }

        // We previously built a mapping from path to path ID and regular
        // expression - see fromOperation for details. Sort it and add an
        // index, and then add it to the objects that we're about to pass to
        // the templates to process.
        // List<Map.Entry<String, Map<String, String>>> pathSetEntryList = new
        // ArrayList(pathSetMap.entrySet());
        // Collections.sort(pathSetEntryList, new Comparator<Map.Entry<String,
        // Map<String, String>>>() {
        // public int compare(Map.Entry<String, Map<String, String>> a,
        // Map.Entry<String, Map<String, String>> b) {
        // return a.getValue().get("path").compareTo(b.getValue().get("path"));
        // }
        // });
        // List pathSet = new ArrayList<Map<String, String>>();
        // int index = 0;
        // for (Map.Entry<String, Map<String, String>> pathSetEntry : pathSetEntryList)
        // {
        // Map<String, String> pathSetEntryValue = pathSetEntry.getValue();
        // pathSetEntryValue.put("index", Integer.toString(index));
        // index++;
        // pathSet.add(pathSetEntryValue);
        // }
        // objs.put("pathSet", pathSet);

        return super.postProcessSupportingFileData(objs);
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
                    // LOGGER.warn("The integer format '{}' is not recognized and will be ignored.",
                    // property.dataFormat);
                    property.datatype = matchingIntType(unsigned, inclusiveMinimum, inclusiveMaximum);
                }
                // switch (property.dataFormat == null ? NO_FORMAT : property.dataFormat) {
                // // standard swagger formats
                // case "int32":
                // property.datatype = unsigned ? "u32" : "i32";
                // break;

                // case "int64":
                // property.datatype = unsigned ? "u64" : "i64";
                // break;

                // case NO_FORMAT:
                // property.datatype = matchingIntType(unsigned, inclusiveMinimum,
                // inclusiveMaximum);
                // break;

                // default:
                // // unknown format
                // LOGGER.warn("The integer format '{}' is not recognized and will be ignored.",
                // property.dataFormat);
                // property.datatype = matchingIntType(unsigned, inclusiveMinimum,
                // inclusiveMaximum);
                // }
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

    private boolean paramHasXmlNamespace(CodegenParameter param, Map<String, Schema> definitions) {
        return false;
        // Object refName = param.vendorExtensions.get("refName");

        // if ((refName != null) && (refName instanceof String)) {
        // String name = (String) refName;
        // Model model = definitions.get(name);

        // if ((model != null) && (model instanceof ModelImpl)) {
        // Xml xml = ((ModelImpl) model).getXml();
        // if ((xml != null) && (xml.getNamespace() != null)) {
        // return true;
        // }
        // }
        // }
        // return false;
    }

    private void processParam(CodegenParameter param, CodegenOperation op) {
        String example = null;

        if (getBooleanValue(param, CodegenConstants.IS_STRING_EXT_NAME)) {
            if (param.dataFormat != null && param.dataFormat.equals("byte")) {
                param.vendorExtensions.put("formatString", "\\\"{:?}\\\"");
                example = "swagger::ByteArray(\"" + ((param.example != null) ? param.example : "")
                        + "\".to_string().into_bytes())";
            } else {
                param.vendorExtensions.put("formatString", "\\\"{}\\\"");
                example = "\"" + ((param.example != null) ? param.example : "") + "\".to_string()";
            }
        } else if (getBooleanValue(param, CodegenConstants.IS_PRIMITIVE_TYPE_EXT_NAME)) {
            if ((getBooleanValue(param, CodegenConstants.IS_BYTE_ARRAY_EXT_NAME))
                    || (getBooleanValue(param, CodegenConstants.IS_BINARY_EXT_NAME))) {
                // Binary primitive types don't implement `Display`.
                param.vendorExtensions.put("formatString", "{:?}");
                example = "swagger::ByteArray(Vec::from(\"" + ((param.example != null) ? param.example : "") + "\"))";
            } else {
                param.vendorExtensions.put("formatString", "{}");
                example = (param.example != null) ? param.example : "";
            }
        } else if (getBooleanValue(param, CodegenConstants.IS_LIST_CONTAINER_EXT_NAME)) {
            param.vendorExtensions.put("formatString", "{:?}");
            example = (param.example != null) ? param.example : "&Vec::new()";
        } else if (getBooleanValue(param, CodegenConstants.IS_FILE_EXT_NAME)) {
            param.vendorExtensions.put("formatString", "{:?}");
            op.vendorExtensions.put("hasFile", true);
            additionalProperties.put("apiHasFile", true);
            example = "Box::new(stream::once(Ok(b\"hello\".to_vec()))) as Box<Stream<Item=_, Error=_> + Send>";
        } else {
            param.vendorExtensions.put("formatString", "{:?}");
            if (param.example != null) {
                example = "serde_json::from_str::<" + param.dataType + ">(\"" + param.example
                        + "\").expect(\"Failed to parse JSON example\")";
            }
        }

        if (param.required) {
            if (example != null) {
                param.vendorExtensions.put("example", example);
            } else if (getBooleanValue(param, CodegenConstants.IS_LIST_CONTAINER_EXT_NAME)) {
                // Use the empty list if we don't have an example
                param.vendorExtensions.put("example", "&Vec::new()");
            } else {
                // If we don't have an example that we can provide, we need to disable the
                // client example, as it won't build.
                param.vendorExtensions.put("example", "???");
                op.vendorExtensions.put("noClientExample", Boolean.TRUE);
            }
        } else if ((param.dataFormat != null)
                && ((param.dataFormat.equals("date-time")) || (param.dataFormat.equals("date")))) {
            param.vendorExtensions.put("formatString", "{:?}");
            param.vendorExtensions.put("example", "None");
        } else {
            // Not required, so override the format string and example
            param.vendorExtensions.put("formatString", "{:?}");
            if (getBooleanValue(param, CodegenConstants.IS_FILE_EXT_NAME)) {
                // Optional file types are wrapped in a future
                param.vendorExtensions.put("example",
                        (example != null)
                                ? "Box::new(future::ok(Some(" + example + "))) as Box<Future<Item=_, Error=_> + Send>"
                                : "None");
            } else {
                param.vendorExtensions.put("example", (example != null) ? "Some(" + example + ")" : "None");
            }
        }
    }

    @Override
    public String getDefaultTemplateDir() {
        return "rust";
    }
}
