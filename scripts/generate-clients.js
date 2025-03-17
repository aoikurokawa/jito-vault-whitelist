const kinobi = require("kinobi");
const anchorIdl = require("@exo-tech-xyz/nodes-from-anchor");
const path = require("path");
const renderers = require('@exo-tech-xyz/renderers');

// Paths.
const projectRoot = path.join(__dirname, "..");

const idlDir = path.join(projectRoot, "idl");

const rustClientsDir = path.join(__dirname, "..", "clients", "rust");

// Generate the restaking client in Rust and JavaScript.
const rustRestakingClientDir = path.join(rustClientsDir, "vault-whitelist-client");
const restakingRootNode = anchorIdl.rootNodeFromAnchor(require(path.join(idlDir, "vault_whitelist.json")));
const restakingKinobi = kinobi.createFromRoot(restakingRootNode);
restakingKinobi.update(kinobi.bottomUpTransformerVisitor([
   {
        // PodU128 -> u128
        select: (node) => {
            return (
                kinobi.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU128"
            );
        },
        transform: (node) => {
            kinobi.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: kinobi.numberTypeNode("u128"),
            };
        },
    },
    {
        // PodU64 -> u64
        select: (node) => {
            return (
                kinobi.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU64"
            );
        },
        transform: (node) => {
            kinobi.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: kinobi.numberTypeNode("u64"),
            };
        },
    },
    {
        // PodU32 -> u32
        select: (node) => {
            return (
                kinobi.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU32"
            );
        },
        transform: (node) => {
            kinobi.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: kinobi.numberTypeNode("u32"),
            };
        },
    },
    {
        // PodU16 -> u16
        select: (node) => {
            return (
                kinobi.isNode(node, "structFieldTypeNode") &&
                node.type.name === "podU16"
            );
        },
        transform: (node) => {
            kinobi.assertIsNode(node, "structFieldTypeNode");
            return {
                ...node,
                type: kinobi.numberTypeNode("u16"),
            };
        },
    },
    // add 8 byte discriminator to accountNode
    {
        select: (node) => {
            return (
                kinobi.isNode(node, "accountNode")
            );
        },
        transform: (node) => {
            kinobi.assertIsNode(node, "accountNode");

            return {
                ...node,
                data: {
                    ...node.data,
                    fields: [
                        kinobi.structFieldTypeNode({ name: 'discriminator', type: kinobi.numberTypeNode('u64') }),
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
