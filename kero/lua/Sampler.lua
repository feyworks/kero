---@meta

---@alias AddressMode "clamp"|"repeat"|"mirror_repeat"

---@alias FilterMode "nearest"|"linear"

---@class (exact) Sampler: SamplerMethods
---@field address_x AddressMode
---@field address_y AddressMode
---@field min_filter FilterMode
---@field mag_filter FilterMode

---@class SamplerClass: SamplerMethods
local module = {}

---@class SamplerMethods
local methods = {}

---The default sampler.
---@return Sampler
---@nodiscard
function module.default() end

---Create a new sampler.
---@param address_x AddressMode
---@param address_y AddressMode
---@param min_filter FilterMode
---@param mag_filter FilterMode
---@return Sampler
---@nodiscard
function module.new(address_x, address_y, min_filter, mag_filter) end

---Create a new sampler with the address and filter modes.
---@param address AddressMode
---@param filter FilterMode
---@return Sampler
---@nodiscard
function module.with(address, filter) end

---Horizontal address mode.
---@param self Sampler
---@return AddressMode
---@nodiscard
function methods.address_x(self) end

---Vertical address mode.
---@param self Sampler
---@return AddressMode
---@nodiscard
function methods.address_y(self) end

---Minified filter mode.
---@param self Sampler
---@return FilterMode
---@nodiscard
function methods.min_filter(self) end

---Magnified filter mode.
---@param self Sampler
---@return FilterMode
---@nodiscard
function methods.mag_filter(self) end

return module