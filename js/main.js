import { Shader, Buffer, Entity } from './webgl.js'


const width = 800
const height = 800


const canvas = document.getElementById('canvas')
canvas.width = width
canvas.height = height
const gl = canvas.getContext('webgl2')



let BUFFER_SIZE
const textDecoder = new TextDecoder()
let wasm
let shaders = new Map()
let buffers = new Map()
let entities = new Map()
let allIds = new Set()


const wasmImports = {
    imports: {

        js_console_log_raw: (ptr, len) => {
            const data = new Uint8Array(wasm.memory.buffer, ptr, len)
            console.log(textDecoder.decode(data))
        },

        js_console_error_raw: (ptr, len) => {
            const data = new Uint8Array(wasm.memory.buffer, ptr, len)
            const error = new Error()
            console.error(
                textDecoder.decode(data) + '\n\n' + error.stack
            )
        },

        js_add_shader: (
            name_ptr, name_len, vert_ptr, vert_len, frag_ptr, frag_len
        ) => {

            let data
            data = new Uint8Array(wasm.memory.buffer, name_ptr, name_len)
            let name = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, vert_ptr, vert_len)
            let vert = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, frag_ptr, frag_len)
            let frag = textDecoder.decode(data)

            const shader = new Shader(vert, frag)
            shader.compile(gl)
            shaders.set(name, shader)
        },


        js_add_buffer_float: (
            name_ptr, name_len, data_ptr, data_len, size, normalize
        ) => {
            let data
            data = new Uint8Array(wasm.memory.buffer, name_ptr, name_len)
            let name = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, data_ptr, data_len)

            const buffer = new Buffer(
                new Float32Array(JSON.parse(textDecoder.decode(data))),
                'STATIC_DRAW',
                size,
                'FLOAT',
                normalize == 0 ? false : true,
            )
            buffer.load(gl)
            buffers.set(name, buffer)
        },

        js_add_buffer_bytes: (
            name_ptr, name_len, data_ptr, data_len, size, normalize
        ) => {
            let data
            data = new Uint8Array(wasm.memory.buffer, name_ptr, name_len)
            let name = textDecoder.decode(data)

            data = new Uint8Array(wasm.memory.buffer, data_ptr, data_len)
            const buffer = new Buffer(
                new Uint8Array(JSON.parse(textDecoder.decode(data))),
                'STATIC_DRAW',
                size,
                'UNSIGNED_BYTE',
                normalize == 0 ? false : true,
            )
            buffer.load(gl)
            buffers.set(name, buffer)
        },

        js_add_entity: (ptr, len) => {
            const data = new Uint8Array(wasm.memory.buffer, ptr, len)
            let meta = JSON.parse(textDecoder.decode(data))

            let id = Math.floor(Math.random() * 999998) + 1
            while( allIds.has(id) ) {
                id = Math.floor(Math.random() * 999998) + 1
            }
            allIds.add(id)

            const entity = new Entity(
                meta.shader, meta.count, meta.attributes, meta.uniforms
            )
            entity.load(gl, shaders, buffers)
            shaders.get(entity.shader).entities.push(id)
            entities.set(id, entity)

            return id
        },

        // js_destroy_entity: (id) => {
        //     entities[id].destroy(gl, shaders[entities[id].shader].program)
        //     const index = shaders[entities[id].shader].entities.indexOf(id)
        //     shaders[entities[id].shader].entities.splice(index, 1)
        //     delete entities[id]
        //     allIds.delete(id)
        // },

    },
}


let keys = 0

async function load() {

    if( !gl ) {
        alert('webgl2 not available!')
        return
    }

    let file = 'target/wasm32-unknown-unknown/debug/neutrino_demo.wasm'
    // let file = 'target/wasm32-unknown-unknown/release/neutrino_demo.wasm'
    wasm = await WebAssembly.instantiateStreaming(fetch(file), wasmImports)
    wasm.memory = wasm.instance.exports.memory
    console.log(wasm)

    BUFFER_SIZE = new Uint32Array(
        WebAssembly.Module.customSections(wasm.module, 'BUFFER_SIZE'
    )[0])[0]

    wasm.ptr = wasm.instance.exports.init()


    const keyShifts = {
        KeyW: 0, KeyA: 1, KeyS: 2, KeyD: 3,
        KeyQ: 4, KeyE: 5, Space: 6,
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

    gl.enable(gl.CULL_FACE)
    gl.enable(gl.DEPTH_TEST)
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height)
    gl.clearColor(0, 0, 0, 1)

    requestAnimationFrame(render)
}
load()



let t = 0
let prevdt, dt
let buffer, bufferptr, bufferLen, bid, blen, b


function render(currentdt) {

    if( prevdt === undefined ) { prevdt = currentdt }
    dt = currentdt - prevdt
    prevdt = currentdt
    dt = Math.floor(dt) % 2 == 0 ? dt : Math.floor(dt) - 1
    dt = dt / 1000

    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

    bufferptr = wasm.instance.exports.render(wasm.ptr, t, dt, keys)
    buffer = new Float32Array(wasm.memory.buffer, bufferptr, BUFFER_SIZE)

    b = 0
    bufferLen = buffer[b++]
    while( b < bufferLen ) {
        bid = buffer[b++]
        blen = buffer[b++]
        entities.get(bid).addFrameUniformUpdate(
            buffer[b++], buffer[b++], buffer.slice(b, b + blen)
        )
        b += blen
    }

    for( const shader of shaders.values() ) {
        gl.useProgram(shader.program)
        for( const entityID of shader.entities ) {
            let entity = entities.get(entityID)
            gl.bindVertexArray(entity.vao)
            entity.updateUniforms(gl, shader.program)
            gl.drawArrays(gl.TRIANGLES, 0, entity.count)
            gl.bindVertexArray(null)
        }
    }

    t += 1
    requestAnimationFrame(render)
}
