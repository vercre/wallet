//
//  store.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 06/11/2024.
//

import Foundation
import SharedTypes
import SwiftData

@Model
final class StoredObject {
    
    // The index or key for the object. Unique for a catalog.
    @Attribute(.unique) let id: String
    
    // The data to be stored as an array of bytes. It's up to the Crux layer to know how to
    // serialize/deserialize into its model.
    let data: Data
    
    init(id: String, data: Data) {
        self.id = id
        self.data = data
    }
    
    convenience init(id: String, bytes: [UInt8]) {
        self.init(id: id, data: Data(bytes))
    }
}

protocol Storer<T> {
    associatedtype T
    func save(_ item: T) throws
    func list() throws -> [T]
    func delete(_ id: String) throws
}

protocol Store<T> : Storer {
    associatedtype T = PersistentModel
    var container: ModelContainer { get }
}

extension Store {
    func save<T: PersistentModel>(_ item: T) throws {
        let context = ModelContext(container)
        context.insert(item)
        try context.save()
    }
    
    func list<T: PersistentModel>() throws -> [T] {
        let context = ModelContext(container)
        return try context.fetch(T.all())
    }
    
    func delete<T: PersistentModel>(_ id: String) throws {
        let context = ModelContext(container)
        try context.delete(model: T.find(id), where: #Predicate { item in item.id == id })
        try context.save()
    }
}

final class ObjectStore: Store {
    typealias T = StoredObject
    let container: ModelContainer
    let catalog: Catalog
    
    // Can use in-memory store for unit testing
    init(catalog: Catalog, useInMemStore: Bool = false) throws {
        let storeName = switch Catalog {
        case .credentials:
            "credential"
        case .issuance:
            "issuance"
        case .presentation:
            "presentation"
        }
        let storeUrl = URL.documentsDirectory.appending(path: "\(storeName).store")
        let config = ModelConfiguration(for: StoredObject.self, name: storeName, url: storeUrl, isStoredInMemoryOnly: useInMemStore)
        self.container = try ModelContainer(for: StoredObject.self, configurations: config)
    }
}

func requestStore(_ request: StoreOperation) async -> Result<StoreReponse, StoreError> {
    do {
        switch request {
        case .save(let catalog, let id, let data):
            let store = try await ObjectStore(catalog: catalog)
            do {
                await store.save(StoredObject(id: id, data: data))
                return .success(.saved)
            } catch {
                return .failure(.invalidResponse(error))
            }
        case .list(let catalog):
            let store = try await ObjectStore(catalog: catalog)
            do {
                let objects = await store.list()
                return .success(.list(objects))
            } catch {
                return .failure(.invalidResponse(error))
            }
        case .delete(catalog: let catalog, id: let id):
            let store = try await ObjectStore(catalog: catalog)
            do {
                await store.delete(id)
                return .success(.deleted)
            } catch {
                return .failure(.invalidResponse(error))
            }
        }
    } catch {
        return .failure(.invalidRequest(error))
    }
}
