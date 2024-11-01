//
//  ContentView.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 15/10/2024.
//

import SharedTypes
import SwiftUI

struct ContentView: View {
    @ObservedObject var core: Core
    @State private var context: Tab = .credentials
    
    enum Tab {
        case credentials
        case issuance
        case presentation
    }
    
    init(core: Core) {
        self.core = core
        core.update(.ready)
        core.update(.startWatch)
    }
    
    var body: some View {
        TabView(selection: $context) {
            CounterView(core: core)
                .tabItem {
                    Label("Counter", systemImage: "chevron.up.chevron.down")
                }
            CredentialList(core: core)
                .tabItem {
                    Label("Credentials", systemImage: "wallet.bifold")
                }
                .tag(Tab.credentials)
            IssuanceScan(core: core)
                .tabItem {
                    Label("Receive", systemImage: "plus.app")
                }
                .tag(Tab.issuance)
            PresentationRequest(core: core)
                .tabItem {
                    Label("Present", systemImage: "checkmark.shield")
                }
                .tag(Tab.presentation)
        }
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(core: Core())
    }
}
