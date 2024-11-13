//
//  signer.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 13/11/2024.
//

import CryptoKit
import Foundation

// Interface to provide conversion between CryptoKit keys and KeyChain items.
protocol GenericPasswordConvertible: CustomStringConvertible {
    // Creates a key from a raw representation.
    init<D>(rawRepresentation data: D) throws where D: ContiguousBytes
    
    // A raw representation of the key.
    var rawRepresentation: Data { get }
}

// ED25519 keys adopt the above interface directly so we simply assert they do.
extension Curve25519.Signing.PrivateKey: @retroactive CustomStringConvertible {}
extension Curve25519.Signing.PrivateKey: GenericPasswordConvertible {}
