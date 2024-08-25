import { getParseTree, getAst } from "../generated/tree_sitter_wasi_web";

const inputElement = document.getElementById("input");
const parseTreeElement = document.getElementById("parse-tree");
const astElement = document.getElementById("ast");

function handleInput(event) {
    parseTreeElement.value = getParseTree(event.target.value);
    astElement.value = getAst(event.target.value);
}

inputElement.addEventListener("input", handleInput);
inputElement.dispatchEvent(new Event('input'));
