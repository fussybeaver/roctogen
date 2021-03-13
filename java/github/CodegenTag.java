package github;

import java.util.ArrayList;
import java.util.List;

import io.swagger.codegen.v3.CodegenContent;
import io.swagger.codegen.v3.CodegenObject;
import io.swagger.codegen.v3.CodegenOperation;

public class CodegenTag extends CodegenObject {
    public String baseName;
    public List<CodegenOperation> operations;
    public List<CodegenContent> contents = new ArrayList<>();
	public String getBaseName() {
		return baseName;
	}
	public void setBaseName(String baseName) {
		this.baseName = baseName;
	}
	public List<CodegenOperation> getOperations() {
		return operations;
	}
	public void setOperations(List<CodegenOperation> operations) {
		this.operations = operations;
	}
	@Override
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + ((baseName == null) ? 0 : baseName.hashCode());
		result = prime * result + ((operations == null) ? 0 : operations.hashCode());
		return result;
	}
	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		CodegenTag other = (CodegenTag) obj;
		if (baseName == null) {
			if (other.baseName != null)
				return false;
		} else if (!baseName.equals(other.baseName))
			return false;
		if (operations == null) {
			if (other.operations != null)
				return false;
		} else if (!operations.equals(other.operations))
			return false;
		return true;
	}
	public List<CodegenContent> getContents() {
		return contents;
	}
	public void setContents(List<CodegenContent> contents) {
		this.contents = contents;
	}
}
