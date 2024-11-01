//
//  CredentialRow.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 01/11/2024.
//

import SharedTypes
import SwiftUI

struct CredentialRow: View {
    var credential: Credential
    
    var body: some View {
        HStack {
            Text(credential.id)
        }
    }
}

//#Preview {
//    CredentialRow()
//}
