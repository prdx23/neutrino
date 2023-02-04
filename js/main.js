import { m4 } from './math.js'
import { shaders, buffers, objects } from './data.js'
import { Shader, Buffer, Object3d } from './webgl.js'


const width = 800
const height = 800

const textDecoder = new TextDecoder()
let wasm
let BUFFER_SIZE

async function run() {

    const importObject = {
        imports: {
            console_log: (ptr, len) => {
                const data = new Uint8Array(
                    wasm.instance.exports.memory.buffer, ptr, len
                )
                console.log(textDecoder.decode(data))
            },
            console_error: (ptr, len) => {
                const data = new Uint8Array(
                    wasm.instance.exports.memory.buffer, ptr, len
                )
                let error = new Error()
                console.error(
                    textDecoder.decode(data) + '\n\n' + error.stack
                )
            }
        },
    }


    let file = 'target/wasm32-unknown-unknown/debug/neutrino_demo.wasm'
    wasm = await WebAssembly.instantiateStreaming(fetch(file), importObject)
    console.log(wasm)
    BUFFER_SIZE = new Uint32Array(
        WebAssembly.Module.customSections(wasm.module, 'BUFFER_SIZE'
    )[0])[0]
    init()
}
run()

let t = 0


function init() {
    let gameptr = wasm.instance.exports.init()

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


    function render() {
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)
        gl.clearColor(0, 0, 0, 1)
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

        // gl.canvas.clientWidth, gl.canvas.clientHeight,
        let bufferptr = wasm.instance.exports.render(gameptr, t)
        const buffer = new Float32Array(
            wasm.instance.exports.memory.buffer, bufferptr, BUFFER_SIZE
        )
        // console.log(buffer)

        let b = 0
        let bufferLen = buffer[b++]

        // let viewProjectionMatrix = buffer.slice(b, b + 16)
        // b += 16

        let data = {}

        let id, len, ublock, uname
        while( b < bufferLen ) {
            id = buffer[b++]
            data[id] = {
                len: buffer[b++],
                ublock: buffer[b++],
                uname: buffer[b++],
            }
            data[id].data = buffer.slice(b, b + data[id].len)
            b += data[id].len
        }


        let i = 0
        for( let shader of Object.values(shaders) ) {
            gl.useProgram(shader.program)

            for( let object of Object.values(shader.objects) ) {
                i += 1
                gl.bindVertexArray(object.vao)

                let matrix = data[i].data
                object.uniformBlocks['objectData'].update(
                    gl, shader.program, 'u_matrix', matrix
                )

                gl.drawArrays(gl.TRIANGLES, 0, object.count)
                gl.bindVertexArray(null)
            }

        }

        t += 1
        requestAnimationFrame(render)
    }
    render()

}
