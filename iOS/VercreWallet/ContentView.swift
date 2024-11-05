//
//  ContentView.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 15/10/2024.
//

import SharedTypes
import SwiftUI

struct ContentView: View {
    @StateObject var core = Core()
    
    init(core: Core) {
        core.update(.ready)
        core.update(.startWatch)
    }
    
    var body: some View {
        NavigationStack() {
            switch core.view.active_view {
            case .credentialList:
                CredentialList(credentials: core.view.credential_view.credentials)
                    .navBar(context: core.view.active_view)
            case .credentialDetail:
                if let credential = core.view.credential_view.credentials.first(where: { $0.id == core.view.credential_view.id }) {
                    CredentialDetailView(credential: credential)
                        .navBar(context: core.view.active_view)
                }
            default : EmptyView()
            }
        }
        .environment(\.update, { e in core.update(e)})
    }
}

private struct UpdateKey: EnvironmentKey {
    static let defaultValue: (Event) -> Void = { _ in }
}

extension EnvironmentValues {
    var update: (Event) -> Void {
        get { self[UpdateKey.self] }
        set { self[UpdateKey.self] = newValue }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(core: Core())
    }
}
