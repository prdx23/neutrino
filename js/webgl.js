
export let Shader = function(vertex, fragment) {

    this.program = null
    this.entities = []

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

    this.name = name
    this.variablesList = variablesList
    this.buffer = null
    this.uboIndex = null
    this.variableOffsets = new Map()

    this.load = function(gl, program, uboIndex) {
        const blockIndex = gl.getUniformBlockIndex(program, this.name)
        const size = gl.getActiveUniformBlockParameter(
            program, blockIndex, gl.UNIFORM_BLOCK_DATA_SIZE
        )

        this.buffer = gl.createBuffer()
        gl.bindBuffer(gl.UNIFORM_BUFFER, this.buffer)
        gl.bufferData(gl.UNIFORM_BUFFER, size, gl.DYNAMIC_DRAW)
        gl.bindBuffer(gl.UNIFORM_BUFFER, null)

        this.uboIndex = uboIndex
        gl.bindBufferBase(gl.UNIFORM_BUFFER, uboIndex, this.buffer)
        gl.uniformBlockBinding(program, blockIndex, uboIndex)

        const variableOffsets = gl.getActiveUniforms(
            program,
            gl.getUniformIndices(program, this.variablesList),
            gl.UNIFORM_OFFSET
        )

        for( const [i, variable] of this.variablesList.entries() ) {
            this.variableOffsets.set(variable, variableOffsets[i])
        }

    }

    this.update = function(gl, program, variable, data) {
        gl.bindBuffer(gl.UNIFORM_BUFFER, this.buffer)
        gl.bufferSubData(
            gl.UNIFORM_BUFFER, this.variableOffsets.get(variable), data, 0
        )
        gl.bindBuffer(gl.UNIFORM_BUFFER, null)
        gl.bindBufferBase(gl.UNIFORM_BUFFER, this.uboIndex, this.buffer)
    }

    // this.destroy = function(gl, program) {
    //     gl.deleteBuffer(this.buffer)
    // }
}


export let Entity = function(shader, count, attributes, uniforms) {

    this.shader = shader
    this.count = count
    this.attributes = attributes
    this.uniforms = uniforms
    this.uniformBlocks = []
    this.frameUniformUpdates = new Map()
    this.vao = null

    this.load = function(gl, shaders, buffers) {
        const program = shaders.get(this.shader).program

        this.vao = gl.createVertexArray()
        gl.bindVertexArray(this.vao)

        for( const attrib of Object.keys(this.attributes) ) {
            const buffer = buffers.get(this.attributes[attrib])

            const location = gl.getAttribLocation(program, attrib)
            gl.enableVertexAttribArray(location)

            gl.bindBuffer(gl.ARRAY_BUFFER, buffer.buffer)
            gl.vertexAttribPointer(
                location, buffer.size, buffer.type, buffer.normalize, 0, 0,
            )
            gl.bindBuffer(gl.ARRAY_BUFFER, null)
        }

        for( const [blockName, variables] of Object.entries(this.uniforms) ) {
            const uniformBlock = new UniformBlock(blockName, variables)
            uniformBlock.load(gl, program, 0)
            this.uniformBlocks.push(uniformBlock)
        }

        gl.bindVertexArray(null)
    }

    this.addFrameUniformUpdate = function(ublockId, unameId, data) {
        const ublock = this.uniformBlocks[ublockId]
        const uname = ublock.variableOffsets.keys()[unameId]
        this.frameUniformUpdates.set(ublockId, [uname, data])
    }

    this.updateUniforms = function(gl, program) {
        for( const [ublockId, [uname, data]] of this.frameUniformUpdates ) {
            this.uniformBlocks[ublockId].update(gl, program, uname, data)
        }
        this.frameUniformUpdates.clear()
    }

    // this.destroy = function(gl, program) {
    //     for( let [blockName, variables] of Object.entries(this.uniforms) ) {
    //         this.uniformBlocks[blockName].destroy(gl, program)
    //     }
    //     for( let attrib of Object.keys(this.attributes) ) {
    //         let location = gl.getAttribLocation(program, attrib)
    //         gl.disableVertexAttribArray(location)
    //     }
    //     gl.deleteVertexArray(this.vao)
    // }

}
