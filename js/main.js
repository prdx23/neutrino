import { m4 } from './math.js'
import { shaders, buffers, objects } from './data.js'
import { Shader, Buffer, Object3d } from './webgl.js'

// import wasmInit from '../pkg/neutrino_demo.js'
// import * as wasm from '../pkg/neutrino_demo.js'


const width = 800
const height = 800

let wasm

async function run() {
    // let w = await wasmInit()

    // game = new wasm.Game()
    // game.init()

    // let ptr = game.test()
    // console.log(ptr, wasm, w)
    // console.log(w.memory)
    // console.log(new Float32Array(w.memory.buffer, ptr, 3))

    const importObject = {
        imports: {
            console_log: (ptr, len) => {
                const decoder = new TextDecoder()
                const data = new Uint8Array(
                    wasm.instance.exports.memory.buffer, ptr, len
                )
                console.log(decoder.decode(data))
            }
        },
    }


    let file = 'target/wasm32-unknown-unknown/debug/neutrino_demo.wasm'
    WebAssembly.instantiateStreaming(fetch(file), importObject).then(
        (results) => {
            wasm = results
            console.log(wasm)
            init()
        }
    )

}
run()



function init() {
    console.log(wasm.instance.exports.test())

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

                let j = i
                // let j = game.get_object(i-1)
                let objectMatrix = m4.identity()
                objectMatrix = m4.yRotate(objectMatrix, -x * 0.5 * j * Math.PI / 180)
                objectMatrix = m4.xRotate(objectMatrix, -x * 0.5 * j * Math.PI / 180)
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
