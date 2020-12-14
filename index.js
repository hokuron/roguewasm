import * as ROT from 'rot-js'
import { Engine, PlayerCore } from './roguewasm'

const Game = {
  display: null,
  engine: null,
  player: null,
  enemy: null,

  init: function () {
    this.display = new ROT.Display({ width: 125, height: 40 })
    document.getElementById('rogueCanvas').appendChild(this.display.getContainer())

    this.engine = new Engine(this.display)
    this.generateMap()

    const scheduler = new ROT.Scheduler.Simple()
    scheduler.add(this.player, true)
    scheduler.add(this.enemy, true)
    this.rotengine = new ROT.Engine(scheduler)
    this.rotengine.start()
  },

  generateMap: function () {
    const digger = new ROT.Map.Digger()
    const freeCells = []
    const digCallback = function (x, y, value) {
      if (!value) {
        const key = x + ',' + y
        freeCells.push(key)
      }
      this.engine.on_dig(x, y, value)
    }
    digger.create(digCallback.bind(this))
    this.generateBoxes(freeCells)
    this.engine.draw_map()
    this.player = this._createBeing(Player, freeCells)
    this.enemy = this._createBeing(Checko, freeCells)
  },

  generateBoxes: function (freeCells) {
    for (let i = 0; i < 10; i++) {
      const index = Math.floor(ROT.RNG.getUniform() * freeCells.length)
      const key = freeCells.splice(index, 1)[0]
      const parts = key.split(',')
      const x = parseInt(parts[0])
      const y = parseInt(parts[1])
      this.engine.place_box(x, y)

      if (i === 9) {
        this.engine.mark_wasmprize(x, y)
      }
    }
  }
}

Game._createBeing = function (What, freeCells) {
  const index = Math.floor(ROT.RNG.getUniform() * freeCells.length)
  const key = freeCells.splice(index, 1)[0]
  const parts = key.split(',')
  const x = parseInt(parts[0])
  const y = parseInt(parts[1])
  return new What(x, y)
}

class Checko {
  constructor (x, y) {
    this._core = new PlayerCore(x, y, 'B', 'red', Game.display)
    this._core.draw()
  }

  act () {
    let x = Game.player.x
    let y = Game.player.y
    const passableCallback = function (x, y) {
      return Game.engine.free_cell(x, y)
    }
    const astar = new ROT.Path.AStar(x, y, passableCallback, { topology: 4 })
    const path = []
    const pathCallback = function (x, y) {
      path.push([x, y])
    }
    astar.compute(this._core.x(), this._core.y(), pathCallback)

    path.shift()
    if (path.length <= 1) {
      Game.rotengine.lock()
      alert('Game over - you were captured by the Borrow Checker!!')
    } else {
      x = path[0][0]
      y = path[0][1]
      Game.engine.move_player(this._core, x, y)
    }
  }
}

class Player {
  constructor (x, y) {
    this._core = new PlayerCore(x, y, '@', '#ff0', Game.display)
    this._core.draw()
  }

  act () {
    Game.rotengine.lock()
    addEventListener('keydown', this)
  }

  handleEvent (e) {
    const keyMap = {}
    keyMap[38] = 0
    keyMap[33] = 1
    keyMap[39] = 2
    keyMap[34] = 3
    keyMap[40] = 4
    keyMap[35] = 5
    keyMap[37] = 6
    keyMap[36] = 7

    const code = e.keyCode

    if (code === 13 || code === 32) {
      Game.engine.open_box(this._core, this._core.x(), this._core.y())
      return
    }

    if (!(code in keyMap)) {
      return
    }

    const dir = ROT.DIRS[8][keyMap[code]]
    const newX = this._core.x() + dir[0]
    const newY = this._core.y() + dir[1]

    if (!Game.engine.free_cell(newX, newY)) {
      return
    }

    Game.engine.move_player(this._core, newX, newY)
    window.removeEventListener('keydown', this)
    Game.rotengine.unlock()
  }

  get x () {
    return this._core.x()
  }

  get y () {
    return this._core.y()
  }
}

Game.init()

export function statsUpdated (stats) {
  document.getElementById('hitpoints').textContent = stats.hitpoints
  document.getElementById('max_hitpoints').textContent = stats.max_hitpoints
  document.getElementById('moves').textContent = stats.moves
}
