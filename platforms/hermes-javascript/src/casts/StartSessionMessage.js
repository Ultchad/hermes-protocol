const ref = require('ref')
const Casteable = require('./Casteable')
const { cast } = require('../tools')
const StringArray = require('./StringArray')
const {
    CStartSessionMessage,
    CSessionInit,
    CActionSessionInit
} = require('../ffi/typedefs')

class StartSessionMessage extends Casteable {
    constructor(args) {
        super(args)
        this.type = CStartSessionMessage
    }

    fromBuffer(buffer) {
        return cast(buffer, {
            session_init: function(sessionInit) {
                const { init_type, value: valuePtr } = sessionInit
                let value = null
                if(init_type === 1) {
                    const actionSessionInitPtr = ref.reinterpret(valuePtr, CActionSessionInit.size)
                    value = cast(ref.get(actionSessionInitPtr, 0, CActionSessionInit), {
                        intent_filter: intents => {
                            return new StringArray(intents)._array
                        }
                    })
                } else if(init_type === 2) {
                    value = valuePtr.readCString()
                }
                return {
                    init_type,
                    value
                }
            }
        })
    }

    forge() {
        return super.forge(this.type, {
            session_init: function(sessionInit) {
                return new Casteable(sessionInit).forge(CSessionInit, {
                    value: value => {
                        if(sessionInit.init_type === 1) {
                            const sessionInitRef = new Casteable(value).forge(CActionSessionInit, {
                                intent_filter: intents => new StringArray(intents).forge()
                            }).ref()
                            ref._attach(sessionInitRef, this)
                            return sessionInitRef
                        } else if(sessionInit.init_type === 2) {
                            const stringPointer = ref.allocCString(value)
                            ref._attach(stringPointer, this)
                            return stringPointer
                        } else {
                            return ref.NULL
                        }
                    }
                })
            }
        })
    }
}

module.exports = StartSessionMessage