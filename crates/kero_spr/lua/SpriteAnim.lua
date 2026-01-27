---@meta

---@alias AnimFrame { duration: number, cels: AnimCel[] }
---@alias AnimCel { layer: integer, index: integer }
---@alias AnimTag { name: string, from: integer, to: integer, dir: AnimDir }
---@alias AnimDir "forward"|"reverse"|"ping_pong"|"ping_pong_reverse"
---@alias AnimLayer { opacity: number, name: string, group: boolean, level: integer }

---A renderable sprite animation.
---@class (exact) SpriteAnim: SpriteAnimMethods

---@class SpriteAnimClass: SpriteAnimMethods
module = {}

---@class SpriteAnimMethods
local methods = {}

---The animation's size (drawing bounds).
---@param self SpriteAnim
---@return Vec2
---@nodiscard
function methods.size(self) end

---The animation's width (drawing bounds).
---@param self SpriteAnim
---@return integer
---@nodiscard
function methods.width(self) end

---The animation's height (drawing bounds).
---@param self SpriteAnim
---@return integer
---@nodiscard
function methods.height(self) end

---The animation's frame count.
---@param self SpriteAnim
---@return integer
---@nodiscard
function methods.num_frames(self) end

---Get the frame at the specified index.
---@param self SpriteAnim
---@param idx integer
---@return AnimFrame?
---@nodiscard
function methods.frame(self, idx) end

---Get the duration of the frame at the specified index.
---@param self SpriteAnim
---@param idx integer
---@return number
---@nodiscard
function methods.frame_duration(self, idx) end

---Get all frames of the animation.
---@param self SpriteAnim
---@return AnimFrame[]
---@nodiscard
function methods.frames(self, idx) end

---The animation's tag count.
---@param self SpriteAnim
---@return integer
---@nodiscard
function methods.num_tags(self) end

---Get the tag at the specified index.
---@param self SpriteAnim
---@param idx integer
---@return AnimTag?
---@nodiscard
function methods.tag(self, idx) end

---Get the tag with the specified name.
---@param self SpriteAnim
---@param name string
---@return AnimTag?
---@nodiscard
function methods.find_tag(self, name) end

---All tags in the animation.
---@param self SpriteAnim
---@return AnimTag[]
---@nodiscard
function methods.tags(self) end

---The animation's layer count.
---@param self SpriteAnim
---@return integer
---@nodiscard
function methods.num_layers(self) end

---Get the layer by index or name.
---@param self SpriteAnim
---@param idx integer|string
---@return AnimLayer?
---@nodiscard
function methods.layer(self, idx) end

---All layers in the animation.
---@param self SpriteAnim
---@return AnimLayer[]
---@nodiscard
function methods.layers(self) end

---Index of the layer with the name.
---@param self SpriteAnim
---@param name string
---@return integer?
---@nodiscard
function methods.layer_idx(self, name) end

---Mask of the layer with the name.
---@param self SpriteAnim
---@param name string
---@return integer?
---@nodiscard
function methods.layer_mask(self, name) end

---Mask of all layers with the names.
---@param self SpriteAnim
---@param names string[]
---@return integer
---@nodiscard
function methods.layer_masks(self, names) end

---Draw a frame of the animation.
---@param self SpriteAnim
---@param frame integer
---@param pos Vec2
---@param layers integer?
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function methods.draw(self, frame, pos, layers, color, mode, flip_x, flip_y) end

return module
