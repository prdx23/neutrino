// import { buffers } from './data.js'
import { Shader, Buffer, Object3d } from './webgl.js'


const width = 800
const height = 800

const textDecoder = new TextDecoder()
let wasm
let t = 0
let keys = 0
let BUFFER_SIZE


let shaders = {}
let buffers = {}
let objects = {}


const wasmImports = {
    imports: {

        console_log_raw: (ptr, len) => {
            const data = new Uint8Array(wasm.memory.buffer, ptr, len)
            console.log(textDecoder.decode(data))
        },

        console_error_raw: (ptr, len) => {
            const data = new Uint8Array(wasm.memory.buffer, ptr, len)
            let error = new Error()
            console.error(
                textDecoder.decode(data) + '\n\n' + error.stack
            )
        },

        add_shader: (
            name_ptr, name_len, vert_ptr, vert_len, frag_ptr, frag_len
        ) => {

            let data
            data = new Uint8Array(wasm.memory.buffer, name_ptr, name_len)
            let name = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, vert_ptr, vert_len)
            let vert = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, frag_ptr, frag_len)
            let frag = textDecoder.decode(data)

            shaders[name] = new Shader(vert, frag)
        },


        add_buffer_float: (
            name_ptr, name_len, data_ptr, data_len, size, normalize
        ) => {
            let data
            data = new Uint8Array(wasm.memory.buffer, name_ptr, name_len)
            let name = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, data_ptr, data_len)
            buffers[name] = new Buffer(
                new Float32Array(JSON.parse(textDecoder.decode(data))),
                'STATIC_DRAW',
                size,
                'FLOAT',
                normalize == 0 ? false : true,
            )
        },

        add_buffer_bytes: (
            name_ptr, name_len, data_ptr, data_len, size, normalize
        ) => {
            let data
            data = new Uint8Array(wasm.memory.buffer, name_ptr, name_len)
            let name = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, data_ptr, data_len)
            buffers[name] = new Buffer(
                new Uint8Array(JSON.parse(textDecoder.decode(data))),
                'STATIC_DRAW',
                size,
                'UNSIGNED_BYTE',
                normalize == 0 ? false : true,
            )
        },

        add_object: (id, ptr, len) => {
            const data = new Uint8Array(wasm.memory.buffer, ptr, len)
            let meta = JSON.parse(textDecoder.decode(data))
            objects[id] = new Object3d(
                meta.shader, meta.count, meta.attributes, meta.uniforms
            )
        },

    },
}



async function load() {
    let file = 'target/wasm32-unknown-unknown/debug/neutrino_demo.wasm'
    // let file = 'target/wasm32-unknown-unknown/release/neutrino_demo.wasm'
    wasm = await WebAssembly.instantiateStreaming(fetch(file), wasmImports)
    wasm.memory = wasm.instance.exports.memory
    console.log(wasm)

    BUFFER_SIZE = new Uint32Array(
        WebAssembly.Module.customSections(wasm.module, 'BUFFER_SIZE'
    )[0])[0]

    init()
}
load()



function init() {
    let ptr = wasm.instance.exports.init()

    const canvas = document.getElementById('canvas')
    canvas.width = width
    canvas.height = height
    const gl = canvas.getContext('webgl2')

    if( !gl ) {
        alert('webgl2 not available!')
        return
    }

    for( let shader of Object.values(shaders) ) {
        shader.compile(gl)
    }

    for( let buffer of Object.values(buffers) ) {
        buffer.load(gl)
    }

    for( let [id, object] of Object.entries(objects) ) {
        object.load(gl, shaders, buffers)
        shaders[object.shader].objects.push(id)
    }

    gl.enable(gl.CULL_FACE)
    gl.enable(gl.DEPTH_TEST)

    let prevdt, dt

    function render(currentdt) {

        if( prevdt === undefined ) { prevdt = currentdt }
        dt = currentdt - prevdt
        prevdt = currentdt
        dt = dt / 10

        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)
        gl.clearColor(0, 0, 0, 1)
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

        // gl.canvas.clientWidth, gl.canvas.clientHeight,
        let bufferptr = wasm.instance.exports.render(ptr, t, dt, keys)
        const buffer = new Float32Array(
            wasm.memory.buffer, bufferptr, BUFFER_SIZE
        )
        // console.log(buffer)

        let b = 0
        let bufferLen = buffer[b++]

        let uniformUpdates = {}
        while( b < bufferLen ) {
            let id = buffer[b++]
            let len = buffer[b++]

            if( !(id in uniformUpdates) ) {
                uniformUpdates[id] = []
            }

            uniformUpdates[id].push({
                ublock: buffer[b++],
                uname: buffer[b++],
                data: buffer.slice(b, b + len)
            })
            b += len
        }
        // console.log(uniformUpdates)


        let i = 0
        for( let shader of Object.values(shaders) ) {
            gl.useProgram(shader.program)

            for( let objectID of shader.objects ) {
                let object = objects[objectID]
                gl.bindVertexArray(object.vao)

                for( let uniformUpdate of uniformUpdates[objectID] ) {
                    object.updateUniform(
                        gl, shader.program,
                        uniformUpdate.ublock,
                        uniformUpdate.uname,
                        uniformUpdate.data,
                    )
                }

                gl.drawArrays(gl.TRIANGLES, 0, object.count)
                gl.bindVertexArray(null)
                i += 1
            }

        }

        t += 1
        requestAnimationFrame(render)
    }
    requestAnimationFrame(render)

}

const keyShifts = {
    KeyW: 0, KeyA: 1, KeyS: 2, KeyD: 3,
}

window.addEventListener("keydown", (event) => {
    if (event.isComposing || event.keyCode === 229 || event.repeat) { return }
    if( Object.keys(keyShifts).includes(event.code) ) {
        keys |= 0b1 << keyShifts[event.code]
    }
    // console.log('key down!', event.code, keys.toString(2), keys)
})

window.addEventListener("keyup", (event) => {
    if( Object.keys(keyShifts).includes(event.code) ) {
        keys &= ~(0b1 << keyShifts[event.code])
    }
    // console.log('key up!', event.code, keys.toString(2), keys)
})
