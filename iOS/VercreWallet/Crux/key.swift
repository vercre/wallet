//
//  key.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 13/11/2024.
//

import Foundation
import Security
import SharedTypes

enum KeyStoreError: Error {
    case generic(Error)
    case message(String)
}

// NOTE: While KeyChain has a kSecClassKey, we use kSecClassGenericPassword to
// store the key bytes as a simple secret. Doing this implies we need a service
// (mapped to 'purpose') and account (mapped to 'id') as the compound key. We
// dispense with specific kSecClassKey storage as this adds complexity around key
// management and types that we avoid since the key value is being used in the
// Crux layer, not directly in Swift itself.

// See: https://www.andyibanez.com/posts/using-ios-keychain-swift/ for a
// reasonable resource for this strange API.

func requestKeyStore(_ request: KeyStoreOperation) async -> Result<KeyStoreResponse, KeyStoreError> {
    switch request {
    case .get(let id, let purpose):
        let query = [
            kSecClass: kSecClassGenericPassword,
            kSecAttrService: purpose,
            kSecAttrAccount: id,
            kSecMatchLimit: kSecMatchLimitOne,
            kSecReturnData: true,
        ] as CFDictionary
        var ref: AnyObject?
        let status = SecItemAdd(query, &ref)
        if status == errSecItemNotFound {
            return .success(.retrieved(key: KeyStoreEntry.none))
        }
        if status != errSecSuccess {
            return .failure(.message("failed to retrieve key: \(status)"))
        }
        let result = ref as! Data
        let entry = KeyStoreEntry.data(Array(result))
        return .success(.retrieved(key: entry))
    case .set(let id, let purpose, let data):
        let query = [
            kSecClass: kSecClassKey,
            kSecAttrService: purpose,
            kSecAttrAccount: id,
            kSecValueData: data
        ] as CFDictionary
        let status = SecItemAdd(query, nil)
        if status != errSecSuccess {
            return .failure(.message("failed to store key: \(status)"))
        }
        return .success(.set)
    case .delete(let id, let purpose):
        let query = [
            kSecClass: kSecClassGenericPassword,
            kSecAttrService: purpose,
            kSecAttrAccount: id
        ] as CFDictionary
        let status = SecItemDelete(query)
        if status != errSecSuccess {
            return .failure(.message("failed to delete key: \(status)"))
        }
        return .success(.deleted)
    case .generateSecret(let length):
        if length > Int.max {
            return .failure(.message("secret length too large: \(length)"))
        }
        let count = Int(length)
        var bytes = [UInt8](repeating: 0, count: count)
        let status = SecRandomCopyBytes(kSecRandomDefault, count, &bytes)
        if status != errSecSuccess {
            return .failure(.message("failed to generate secret: \(status)"))
        }
        return .success(.generatedSecret(secret: Array(bytes)))
    }
}
