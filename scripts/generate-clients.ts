import * as kinobi from "kinobi";
import * as anchorIdl from "@exo-tech-xyz/nodes-from-anchor";
import * as path from "path";
import * as renderers from '@exo-tech-xyz/renderers';

// Paths.
const projectRoot = path.join(__dirname, "..");
const idlDir = path.join(projectRoot, "idl");
const rustClientsDir = path.join(__dirname, "..", "clients", "rust");

// Generate the restaking client in Rust and JavaScript.
const rustRestakingClientDir = path.join(rustClientsDir, "vault-whitelist-client");
// Use require for IDL to avoid type issues
const jitoVaultWhitelistIdl = require(path.join(idlDir, "jito_vault_whitelist.json"));
const restakingRootNode = anchorIdl.rootNodeFromAnchor(jitoVaultWhitelistIdl);
// Cast to any to bypass strict type checking between library versions
const restakingKinobi = kinobi.createFromRoot(restakingRootNode as any);

// Using type assertions to overcome type compatibility issues
restakingKinobi.update(kinobi.bottomUpTransformerVisitor([
  {
    // PodU128 -> u128
    select: (node: any): boolean => {
      return (
        kinobi.isNode(node, "structFieldTypeNode") &&
        node.type && 
        // Check if type exists and has a name property
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU128"
      );
    },
    transform: (node: any): any => {
      kinobi.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: kinobi.numberTypeNode("u128"),
      };
    },
  },
  {
    // PodU64 -> u64
    select: (node: any): boolean => {
      return (
        kinobi.isNode(node, "structFieldTypeNode") &&
        node.type && 
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU64"
      );
    },
    transform: (node: any): any => {
      kinobi.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: kinobi.numberTypeNode("u64"),
      };
    },
  },
  {
    // PodU32 -> u32
    select: (node: any): boolean => {
      return (
        kinobi.isNode(node, "structFieldTypeNode") &&
        node.type && 
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU32"
      );
    },
    transform: (node: any): any => {
      kinobi.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: kinobi.numberTypeNode("u32"),
      };
    },
  },
  {
    // PodU16 -> u16
    select: (node: any): boolean => {
      return (
        kinobi.isNode(node, "structFieldTypeNode") &&
        node.type && 
        typeof node.type === 'object' && 
        'name' in node.type &&
        node.type.name === "podU16"
      );
    },
    transform: (node: any): any => {
      kinobi.assertIsNode(node, "structFieldTypeNode");
      return {
        ...node,
        type: kinobi.numberTypeNode("u16"),
      };
    },
  },
  // add 8 byte discriminator to accountNode
  {
    select: (node: any): boolean => {
      return (
        kinobi.isNode(node, "accountNode")
      );
    },
    transform: (node: any): any => {
      kinobi.assertIsNode(node, "accountNode");
      
      // Using a more direct approach to avoid accessing fields directly
      const discriminator = kinobi.structFieldTypeNode({ 
        name: 'discriminator', 
        type: kinobi.numberTypeNode('u64') 
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
restakingKinobi.accept(renderers.renderRustVisitor(
  path.join(rustRestakingClientDir, "src", "generated"),
  {
    formatCode: true,
    crateFolder: rustRestakingClientDir,
    deleteFolderBeforeRendering: true,
    toolchain: "+nightly-2024-07-25"
  }
));