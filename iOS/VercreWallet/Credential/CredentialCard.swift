//
//  CredentialCard.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 01/11/2024.
//

import SharedTypes
import SwiftUI

struct CredentialCard: View {
    var body: some View {
        Text(/*@START_MENU_TOKEN@*/"Hello, World!"/*@END_MENU_TOKEN@*/)
    }
}

#Preview {
    static let credential = Credential(
        id: "http://vercre.io/credentials/EmployeeIDCredential",
        issuer: "http://vercre.io",
        issued: "encoded",
        type: ["VerifiableCredential", "EmployeeIDCredential"],
        format: "jwt_vc_json",
        claims: ["did:key:z6Mkj8Jr1rg3YjVWWhg7ahEYJibqhjBgZt1pDCbT4Lv7D4HX": "Address: \n  Locality: Wellington\n  Street_address: 123 Fake St\nEmail: normal.user@example.com\nFamily name: Person\nGiven name: Normal\n"],
        issuance_date: <#T##String#>
    )
    CredentialCard()
}
