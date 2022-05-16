"use strict";
((window) => {
    const core = Deno.core;
    function getConfig(){
        return core.opSync("op_config");
    }
    function getAllDict(){
        return core.opSync("op_all_dict");
    }
    function getUserById(id){
        return core.opSync("op_user_info",id);
    }
    const finalCassieOP = {
        getConfig:getConfig,
        getAllDict:getAllDict,
        getUserById:getUserById
      };
    globalThis.Cassie = finalCassieOP;
})(this);