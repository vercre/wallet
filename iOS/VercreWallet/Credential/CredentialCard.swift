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
    static let claims = SubjectClaims(
        id: "did:key:z6Mkj8Jr1rg3YjVWWhg7ahEYJibqhjBgZt1pDCbT4Lv7D4HX",
        claims: [
            "given_name": "Normal",
            "family_name": "Person",
            "email": "normal.user@example.com",
            "address": "{\"street_address\": \"123 Fake St\", \"locality\": \"Wellington\"}"
        ]
    )
    static let credential = Credential(
        id: "http://vercre.io/credentials/EmployeeIDCredential",
        issuer: "http://vercre.io",
        issued: "encoded",
        type: ["VerifiableCredential", "EmployeeIDCredential"],
        format: "jwt_vc_json"
    )
    CredentialCard()
}
