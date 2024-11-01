//
//  CredentialList.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 21/10/2024.
//

import SharedTypes
import SwiftUI

struct CredentialList: View {
    @ObservedObject var core: Core
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Credentials")
                .font(.title)
                .fontWeight(.bold)
            List(core.view.credential_view.credentials, id: \.id) { credential in
                    CredentialRow(credential: credential)
            }
            Spacer()
        }
        .padding()
    }
}

#Preview {
    CredentialList(core: Core())
}
