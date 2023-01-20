const width = 800
const height = 800


init()

function init() {

    const canvas = document.getElementById('canvas')
    canvas.width = width
    canvas.height = height
    const gl = canvas.getContext('webgl2')

    if( !gl ) {
        alert('webgl2 not available!')
        return
    }

    for( let [name, shader] of Object.entries(shaders) ) {
        shaders[name] = new Shader(shader.vertex, shader.fragment)
        shaders[name].compile(gl)
    }

    for( let [name, bufferData] of Object.entries(buffers) ) {
        buffers[name] = new Buffer(...Object.values(bufferData))
        buffers[name].load(gl)
    }

    for( let [name, object] of Object.entries(objects) ) {
        objects[name] = new Object3d(...Object.values(object))
        objects[name].load(gl, shaders, buffers)
        shaders[object.shader].objects.push(objects[name])
    }

    gl.enable(gl.CULL_FACE)
    gl.enable(gl.DEPTH_TEST)

    let x = 0

    function render() {
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)
        gl.clearColor(0, 0, 0, 1)
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)


        let cameraMatrix = m4.identity()
        cameraMatrix = m4.translate(cameraMatrix, 0, 300, 1800)
        cameraMatrix = m4.lookAt(
            [cameraMatrix[12], cameraMatrix[13], cameraMatrix[14]],
            [0, 0, 0],
            [0, 1, 0],
        )

        let viewMatrix = m4.inverse(cameraMatrix)

        let projectionMatrix = m4.perspective(
            30 * Math.PI / 180,
            gl.canvas.clientWidth / gl.canvas.clientHeight,
            1, 2000,
        )

        let viewProjectionMatrix = m4.multiply(projectionMatrix, viewMatrix)


        let i = 0
        for( let shader of Object.values(shaders) ) {
            gl.useProgram(shader.program)

            for( let object of Object.values(shader.objects) ) {
                i += 1
                gl.bindVertexArray(object.vao)

                let objectMatrix = m4.identity()
                objectMatrix = m4.yRotate(objectMatrix, -x * 0.5 * i * Math.PI / 180)
                objectMatrix = m4.xRotate(objectMatrix, -x * 0.5 * i * Math.PI / 180)
                let matrix = object.uniforms.u_matrix.update(
                    viewProjectionMatrix, objectMatrix
                )
                object.uniformBlocks['objectData'].update(
                    gl, shader.program, 'u_matrix', new Float32Array(matrix)
                )

                gl.drawArrays(gl.TRIANGLES, 0, object.count)
                gl.bindVertexArray(null)
            }

        }

        x += 1
        requestAnimationFrame(render)
    }
    render()

}
