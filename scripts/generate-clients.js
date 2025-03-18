const codama = require("codama");
const anchorIdl = require("@codama/nodes-from-anchor");
const path = require("path");
const renderers = require('@codama/renderers');

// Paths.
const projectRoot = path.join(__dirname, "..");

const idlDir = path.join(projectRoot, "idl");

const rustClientsDir = path.join(__dirname, "..", "clients", "rust");

// Generate the restaking client in Rust and JavaScript.
const rustRestakingClientDir = path.join(rustClientsDir, "vault-whitelist-client");
const restakingRootNode = anchorIdl.rootNodeFromAnchor(require(path.join(idlDir, "vault_whitelist.json")));
const restakingKinobi = codama.createFromRoot(restakingRootNode);
restakingKinobi.update(codama.bottomUpTransformerVisitor([
   {
        // PodU128 -> u128
        select: (node) => {
            return (
                codama.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU128"
            );
        },
        transform: (node) => {
            codama.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: codama.numberTypeNode("u128"),
            };
        },
    },
    {
        // PodU64 -> u64
        select: (node) => {
            return (
                codama.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU64"
            );
        },
        transform: (node) => {
            codama.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: codama.numberTypeNode("u64"),
            };
        },
    },
    {
        // PodU32 -> u32
        select: (node) => {
            return (
                codama.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU32"
            );
        },
        transform: (node) => {
            codama.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: codama.numberTypeNode("u32"),
            };
        },
    },
    {
        // PodU16 -> u16
        select: (node) => {
            return (
                codama.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU16"
            );
        },
        transform: (node) => {
            codama.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: codama.numberTypeNode("u16"),
            };
        },
    },
    // add 8 byte discriminator to accountNode
    {
        select: (node) => {
            return (
                codama.isNode(node, "accountNode")
            );
        },
        transform: (node) => {
            codama.assertIsNode(node, "accountNode");

            return {
                ...node,
                data: {
                    ...node.data,
                    fields: [
                        codama.structFieldTypeNode({ name: 'discriminator', type: codama.numberTypeNode('u64') }),
                        ...node.data.fields
                    ]
                }
            };
        },
    },
]));
restakingKinobi.accept(renderers.renderRustVisitor(path.join(rustRestakingClientDir, "src", "generated"), {
    formatCode: true,
    crateFolder: rustRestakingClientDir,
    deleteFolderBeforeRendering: true,
    toolchain: "+nightly-2024-07-25"
}));
