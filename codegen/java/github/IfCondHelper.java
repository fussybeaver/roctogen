package github;

import java.io.IOException;

import com.github.jknack.handlebars.Options;
import com.google.common.base.Objects;

public class IfCondHelper {
    public CharSequence equals(final Object obj1, final Options options) throws IOException {
        Object obj2 = options.param(0);
        return Objects.equal(obj1, obj2) ? options.fn() : options.inverse();
    }
}
