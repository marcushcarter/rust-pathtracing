pub const RT_COMPUTE_SRC: &str = r#"
    #version 460 core
    layout (local_size_x = 8, local_size_y = 8) in;
    layout (rgba32f, binding = 0) uniform image2D uOutput;

    uniform vec2  uResolution;
    uniform vec3  uCamPos;
    uniform vec3  uCamForward;
    uniform vec3  uCamRight;
    uniform vec3  uCamUp;
    uniform float uTanHalfFov;

    void main() {
        ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
        if (pixel.x >= int(uResolution.x) || pixel.y >= int(uResolution.y)) return;

        vec2 uv = (vec2(pixel) + 0.5) / uResolution * 2.0 - 1.0; // NDC [-1, 1]
        float aspect = uResolution.x / uResolution.y;

        vec3 ro = uCamPos;
        vec3 rd = normalize(
            uCamForward
            + uv.x * aspect * uTanHalfFov * uCamRight
            + uv.y * uTanHalfFov * uCamUp
        );

        vec3 oc = ro - vec3(0.0);
        float a = dot(rd, rd);
        float b = 2.0 * dot(oc, rd);
        float c = dot(oc, oc) - 0.25;
        float disc = b * b - 4.0 * a * c;
            
        // vec3 color;
        // if (disc > 0.0) {
        //     float t = (-b - sqrt(disc)) / (2.0 * a);
        //     if (t > 0.0) {
        //         vec3 hitPos = ro + rd * t;
        //         vec3 normal = normalize(hitPos);
        //         color = normal * 0.5 + 0.5;
        //     } else {
        //         color = vec3(0.0);
        //     }
        // } else {
        //     color = rd;
        // }

        vec3 color = disc > 0.0 ? vec3(1.0, 0.0, 0.0) : rd;
        imageStore(uOutput, pixel, vec4(color, 1.0));
    }
"#;

pub const BLIT_VERT_SRC: &str = r#"
    #version 460 core
    out vec2 vUV;
    void main() {
        vec2 uv = vec2((gl_VertexID << 1) & 2, gl_VertexID & 2);
        vUV = uv;
        gl_Position = vec4(uv * 2.0 - 1.0, 0.0, 1.0);
    }
"#;

pub const BLIT_FRAG_SRC: &str = r#"
    #version 460 core
    in vec2 vUV;
    out vec4 FragColor;
    layout (binding = 0) uniform sampler2D uImage;
    void main() {
        FragColor = texture(uImage, vUV);
    }
"#;