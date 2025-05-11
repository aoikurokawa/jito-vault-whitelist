import * as codama from "codama";
import * as anchorIdl from "@codama/nodes-from-anchor";
import * as path from "path";
import * as renderers from '@codama/renderers';

// Paths.
const projectRoot = path.join(__dirname, "..");

const idlDir = path.join(projectRoot, "idl");

const rustClientsDir = path.join(__dirname, "..", "client");
const jsClientsDir = path.join(__dirname, "..", "packages", "src");

// Generate the vault whitelist client in Rust and JavaScript.
const vaultWhitelistIdl = require(path.join(idlDir, "jito_vault_whitelist.json"));
const vaultWhitelistRootNode = anchorIdl.rootNodeFromAnchor(vaultWhitelistIdl);
// Cast to any to bypass strict type checking between library versions
const vaultWhitelistCodama = codama.createFromRoot(vaultWhitelistRootNode as any);

// Using type assertions to overcome type compatibility issues
vaultWhitelistCodama.update(codama.bottomUpTransformerVisitor([
  {
    // PodU128 -> u128
    select: (node: any): boolean => {
      return (
        codama.isNode(node, "structFieldTypeNode") &&
        node.type && 
        // Check if type exists and has a name property
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU128"
      );
    },
    transform: (node: any): any => {
      codama.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: codama.numberTypeNode("u128"),
      };
    },
  },
  {
    // PodU64 -> u64
    select: (node: any): boolean => {
      return (
        codama.isNode(node, "structFieldTypeNode") &&
        node.type && 
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU64"
      );
    },
    transform: (node: any): any => {
      codama.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: codama.numberTypeNode("u64"),
      };
    },
  },
  {
    // PodU32 -> u32
    select: (node: any): boolean => {
      return (
        codama.isNode(node, "structFieldTypeNode") &&
        node.type && 
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU32"
      );
    },
    transform: (node: any): any => {
      codama.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: codama.numberTypeNode("u32"),
      };
    },
  },
  {
    // PodU16 -> u16
    select: (node: any): boolean => {
      return (
        codama.isNode(node, "structFieldTypeNode") &&
        node.type && 
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU16"
      );
    },
    transform: (node: any): any => {
      codama.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: codama.numberTypeNode("u16"),
      };
    },
  },
  // add 8 byte discriminator to accountNode
  {
    select: (node: any): boolean => {
      return (
        codama.isNode(node, "accountNode")
      );
    },
    transform: (node: any): any => {
      codama.assertIsNode(node, "accountNode");
      
      // Using a more direct approach to avoid accessing fields directly
      const discriminator = codama.structFieldTypeNode({ 
        name: 'discriminator', 
        type: codama.numberTypeNode('u64') 
      });
      
      // Create a copy of the node with the new discriminator field
      // Access the data structure more carefully
      if (node.data) {
        // Create a modified data object
        const modifiedData = { ...node.data };
        
        // Access fields through data structure with TypeScript type coercion
        if ((node.data as any).fields) {
          // @ts-ignore - Intentionally ignoring type errors due to incompatible library versions
          modifiedData.fields = [
            discriminator,
            ...((node.data as any).fields || [])
          ];
        } else {
          // If fields don't exist, create them with just the discriminator
          // @ts-ignore - Intentionally ignoring type errors due to incompatible library versions
          modifiedData.fields = [discriminator];
        }
        
        return {
          ...node,
          data: modifiedData
        };
      }
      
      // If somehow data doesn't exist, return the node unchanged
      return node;
    },
  },
]));

// Completely bypass type checking for the renderRustVisitor call
// @ts-ignore - Intentionally ignoring type errors due to incompatible library versions
vaultWhitelistCodama.accept(renderers.renderRustVisitor(
  path.join(rustClientsDir, "src", "generated"),
  {
    formatCode: true,
    crateFolder: rustClientsDir,
    deleteFolderBeforeRendering: true,
    toolchain: "+nightly-2024-07-25"
  }
));
vaultWhitelistCodama.accept(renderers.renderJavaScriptVisitor(path.join(jsClientsDir), {}));