pub const RT_COMPUTE_SRC: &str = r#"
    #version 460 core
    layout (local_size_x = 8, local_size_y = 8) in;

    layout (rgba32f, binding = 0) uniform image2D uOutput;

    layout (std140, binding = 0) uniform Camera {
        vec3  uCamPos;
        float uTanHalfFov;
        vec3  uCamForward;
        vec3  uCamRight;
        vec3  uCamUp;
        vec2  uResolution;
    };

    struct Sphere {
        vec3  center;
        float radius;
        vec3  albedo;
        float fuzz;
        int   matType;
        float ior;
        vec2  _pad;
    };

    layout (std430, binding = 0) readonly buffer Spheres {
        Sphere spheres[];
    };

    void main() {
        ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
        if (pixel.x >= int(uResolution.x) || pixel.y >= int(uResolution.y)) return;

        vec2 uv = (vec2(pixel) + 0.5) / uResolution * 2.0 - 1.0; // NDC [-1, 1]
        float aspect = uResolution.x / uResolution.y;

        vec3 ro = uCamPos;
        vec3 rd = normalize(uCamForward + uv.x * aspect * uTanHalfFov * uCamRight + uv.y * uTanHalfFov * uCamUp);

        float tClosest = 1e30;
        bool  hit = false;

        for (int i = 0; i < spheres.length(); i++) {
            vec3  oc = ro - spheres[i].center;
            float a  = dot(rd, rd);
            float b  = 2.0 * dot(oc, rd);
            float c  = dot(oc, oc) - spheres[i].radius * spheres[i].radius;
            float disc = b * b - 4.0 * a * c;
            if (disc < 0.0) continue;

            float t = (-b - sqrt(disc)) / (2.0 * a); // near root
            if (t > 0.001 && t < tClosest) {
                tClosest = t;
                hit = true;
            }
        }

        vec3 color = hit ? vec3(1.0, 0.0, 0.0) : rd * 0.5 + 0.5;
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