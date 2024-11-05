//
//  NavigationBar.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 05/11/2024.
//

import SharedTypes
import SwiftUI

struct NavigationBar: ToolbarContent {
    let context: Aspect
    
    @Environment(\.update) var update
    
    var body: some ToolbarContent {
        ToolbarItemGroup(placement: .bottomBar){
            Button(action: {
                update(.navigate(.credentialList))
            }) {
                VStack {
                    Image(systemName: "wallet.bifold")
                    Text("Credentials").font(.caption2)
                }
            }.disabled(
                context == .init(.credentialList)
            )
            Spacer()
            Button(action: {
                update(.navigate(.issuanceScan))
            }) {
                VStack {
                    Image(systemName: "plus.app")
                    Text("Receive").font(.caption2)
                }
            }.disabled(
                context == .init(.issuanceScan)
            )
            Spacer()
            Button(action: {
                update(.navigate(.presentationScan))
            }) {
                VStack {
                    Image(systemName: "checkmark.shield")
                    Text("Present").font(.caption2)
                }
            }.disabled(
                context == .init(.presentationScan)
            )
        }
    }
}

struct NavigationBarModifier: ViewModifier {
    let context: Aspect
    
    func body(content: Content) -> some View {
        return content
            .toolbar {
                NavigationBar(context: context)
            }
            .navigationBarBackButtonHidden(true)
    }
}

extension View {
    func navBar(context: Aspect) -> some View {
        return self.modifier(NavigationBarModifier(context: context))
    }
}

#Preview {
    VStack {
        Spacer()
    }.toolbar {
        NavigationBar(context: .credentialDetail)
    }
    .navigationBarBackButtonHidden(true)
}
