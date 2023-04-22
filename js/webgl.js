
export let Shader = function(vertex, fragment) {

    this.program = null
    this.objects = []

    this.compile = function(gl) {
        function createShader(type, source) {
            let shader = gl.createShader(type)
            gl.shaderSource(shader, source)
            gl.compileShader(shader)

            if( !gl.getShaderParameter(shader, gl.COMPILE_STATUS) ) {
                console.error('Shader Error: ', gl.getShaderInfoLog(shader))
                gl.deleteShader(shader)
            }

            return shader
        }

        let vertexShader = createShader(gl.VERTEX_SHADER, vertex)
        let fragmentShader = createShader(gl.FRAGMENT_SHADER, fragment)

        let program = gl.createProgram()
        gl.attachShader(program, vertexShader)
        gl.attachShader(program, fragmentShader)
        gl.linkProgram(program)

        if( !gl.getProgramParameter(program, gl.LINK_STATUS) ) {
            console.error('Program Error: ', gl.getProgramInfoLog(program))
            gl.deleteProgram(program)
        }

        this.program = program
    }
}


export let Buffer = function(data, drawType, size, type, normalize) {

    this.data = data
    this.drawType = drawType
    this.size = size
    this.type = type
    this.normalize = normalize
    this.buffer = null

    this.load = function(gl) {
        this.drawType = gl[this.drawType]
        this.type = gl[this.type]

        let buffer = gl.createBuffer()
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer)
        gl.bufferData(gl.ARRAY_BUFFER, this.data, this.drawType)
        gl.bindBuffer(gl.ARRAY_BUFFER, null)
        this.buffer = buffer
    }
}


export let UniformBlock = function(name, variablesList) {

    this.buffer = null
    this.uboIndex = null
    this.variables = {}

    this.load = function(gl, program, uboIndex) {
        let blockIndex = gl.getUniformBlockIndex(program, name)
        let size = gl.getActiveUniformBlockParameter(
            program, blockIndex, gl.UNIFORM_BLOCK_DATA_SIZE
        )

        this.buffer = gl.createBuffer()
        gl.bindBuffer(gl.UNIFORM_BUFFER, this.buffer)
        gl.bufferData(gl.UNIFORM_BUFFER, size, gl.DYNAMIC_DRAW)
        gl.bindBuffer(gl.UNIFORM_BUFFER, null)

        this.uboIndex = uboIndex
        gl.bindBufferBase(gl.UNIFORM_BUFFER, this.uboIndex, this.buffer)
        gl.uniformBlockBinding(program, this.blockIndex, this.uboIndex)

        let variableIndices = gl.getUniformIndices(program, variablesList)
        let variableOffsets = gl.getActiveUniforms(
            program, variableIndices, gl.UNIFORM_OFFSET
        )

        this.variables = {}
        for( let [i, variable] of variablesList.entries() ) {
            this.variables[variable] = {
                index: variableIndices[i],
                offset: variableOffsets[i],
            }
        }
    }

    this.update = function(gl, program, variable, data) {
        gl.bindBuffer(gl.UNIFORM_BUFFER, this.buffer)
        gl.bufferSubData(
            gl.UNIFORM_BUFFER, this.variables[variable].offset, data, 0
        )
        gl.bindBuffer(gl.UNIFORM_BUFFER, null)
        gl.bindBufferBase(gl.UNIFORM_BUFFER, this.uboIndex, this.buffer)
    }

}


export let Object3d = function(shader, count, attributes, uniforms) {

    this.shader = shader
    this.count = count
    this.attributes = attributes
    this.uniforms = uniforms
    this.uniformBlocks = {}
    this.vao = null

    this.load = function(gl, shaders, buffers) {
        let program = shaders[this.shader].program

        this.vao = gl.createVertexArray()
        gl.bindVertexArray(this.vao)

        for( let attrib of Object.keys(this.attributes) ) {
            let buffer = buffers[this.attributes[attrib]]

            let location = gl.getAttribLocation(program, attrib)
            gl.enableVertexAttribArray(location)

            gl.bindBuffer(gl.ARRAY_BUFFER, buffer.buffer)
            gl.vertexAttribPointer(
                location, buffer.size, buffer.type, buffer.normalize, 0, 0,
            )
            gl.bindBuffer(gl.ARRAY_BUFFER, null)
        }

        for( let [blockName, variables] of Object.entries(this.uniforms) ) {
            this.uniformBlocks[blockName] = new UniformBlock(blockName, variables)
            this.uniformBlocks[blockName].load(gl, program, 0)
        }

        gl.bindVertexArray(null)
    }

    this.updateUniform = function(gl, program, ublockId, unameId, data) {
        let ublock = Object.values(this.uniformBlocks)[ublockId]
        let uname = Object.keys(ublock.variables)[unameId]
        ublock.update(gl, program, uname, data)
    }

}
