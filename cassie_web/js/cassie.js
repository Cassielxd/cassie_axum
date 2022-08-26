"use strict";
((window) => {
    const core = Deno.core;
    //获取 系统配置 Cassie.getConfig();
    function getConfig(){
        return core.opSync("op_config");
    }
    //获取 所有系统字典 Cassie.getAllDict();
    function getAllDict(){
        return core.opSync("op_all_dict");
    }
    //获取 用户信息 Cassie.getUserById();
    function getUserById(id){
        return core.opSync("op_user_info",id);
    }
    //想要访问 直接使用 Cassie.XX()的方式
    globalThis.Cassie = {
                getConfig:getConfig,
                getAllDict:getAllDict,
                getUserById:getUserById
                };
})(this);