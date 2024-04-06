#[macro_export]
macro_rules! verify (
    ($e: expr) => ({
        let result = $e;
        let error_code = gl::GetError();
        if error_code != 0 {
            let description = match error_code {
                gl::INVALID_ENUM =>                  "GL_INVALID_ENUM",
                gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                gl::INVALID_OPERATION =>             "GL_INVALID_OPERATION",
                gl::INVALID_VALUE =>                 "GL_INVALID_VALUE",
                gl::NO_ERROR =>                      "GL_NO_ERROR",
                gl::OUT_OF_MEMORY =>                 "GL_OUT_OF_MEMORY",
                _ => panic!("Bad gl error code: {}", error_code),
            };
            panic!("gl error: {}({})", description, error_code);
        }
        result
    })
);
