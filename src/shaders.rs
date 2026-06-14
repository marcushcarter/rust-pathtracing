 
pub const RTX_VERT_SRC: &str = r#"
    #version 460 core
    layout (location = 0) in vec3 aPos;
    out vec2 vUV;
    void main() {
        gl_Position = vec4(aPos, 1.0);
        vUV = vec2(aPos.x * 0.5 + 0.5, 1.0 - (aPos.y * 0.5 + 0.5));
    }
"#;

pub const RTX_FRAG_SRC: &str = r#"
    #version 460 core
    in vec2 vUV;
    out vec4 FragColor;
    void main() {
        // ray origin and direction from UV
        vec2 uv = vUV * 2.0 - 1.0; // remap to [-1, 1]
        vec3 ro = vec3(0.0, 0.0, 2.0);
        vec3 rd = normalize(vec3(uv, -1.0));

        // sphere at origin, radius 0.5
        vec3 oc = ro;
        float a = dot(rd, rd);
        float b = 2.0 * dot(oc, rd);
        float c = dot(oc, oc) - 0.5 * 0.5;
        float disc = b*b - 4.0*a*c;

        if (disc > 0.0) {
            FragColor = vec4(1.0, 0.0, 0.0, 1.0); // hit
        } else {
            FragColor = vec4(0.0, 0.0, 0.0, 1.0); // miss
        }
    }
"#;