Scene = {}
Scene.__index = Scene

function Scene:new()
    local scene = {
        -- Some state here
    }
    setmetatable(scene, Scene)
    return scene
end

function Scene.on_init()
    print("Init")
end

function Scene:on_update(dt)
    print("Delta time"..dt.."s")
end

return Scene:new()
