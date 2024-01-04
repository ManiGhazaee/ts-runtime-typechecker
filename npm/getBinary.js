const { Binary } = require("binary-install");
const os = require("os");

function getPlatform() {
    const type = os.type();
    const arch = os.arch();

    if (type === "Windows_NT" && arch === "x64") return "win64";
    if (type === "Linux" && arch === "x64") return "linux";
    if (type === "Darwin" && arch === "x64") return "macos";

    throw new Error(
        `Unsupported platform: ${type} ${arch}. Please create an issue at https://github.com/ManiGhazaee/ts-runtime-typechecker/issues`
    );
}

function getBinary() {
    const version = require("../package.json").version;
    const platform = getPlatform();
    const url = `https://github.com/ManiGhazaee/ts-runtime-typechecker/releases/download/v${version}/ts-runtime-typechecker-${platform}.tar.gz`;
    console.log(url);
    const name = "ts-runtime-typechecker";
    return new Binary(url, { name });
}

module.exports = getBinary;
