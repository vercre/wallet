//
//  OfferView.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 21/11/2024.
//

import SharedTypes
import SwiftUI

struct OfferCard: View {
    var offer: CredentialSummary
    var issuer: String
    
    var body: some View {
        let txtColor = offer.text_color.isEmpty ? "#000000" : offer.text_color
        let bgColor = offer.background_color.isEmpty ? "#FFFFFF" : offer.background_color

        ZStack(alignment: .topLeading) {
            RoundedRectangle(cornerRadius: 10).size(width: 300, height: 190)
                .fill(Color(UIColor(hex: bgColor)))
                .shadow(radius: 10)
            VStack(alignment: .leading) {
                HStack(alignment: .top){
                    Spacer()
                    Text(offer.name)
                        .font(.headline)
                        .foregroundStyle(Color(UIColor(hex: txtColor)))
                }
                Spacer()
                Text(issuer)
                    .foregroundStyle(Color(UIColor(hex: txtColor)))
            }
            .padding()
        }.frame(width: 300, height: 190)
    }
}

#Preview {
    let offer = CredentialSummary(
        config_id: "EmployeeID_JWT",
        name: "Employee ID",
        description: "Vercre employee ID credential",
        claims: [
            "Email",
            "Family name",
            "Given name",
            "Address.Street Address",
            "Address.Locality",
            "Address.Region",
            "Address.Country"
        ],
        background_color: "#323ed2",
        text_color: "#ffffff",
        logo_url: "https://vercre.github.io/assets/employee.png",
        background_url: "https://vercre.github.io/assets/employee-background.png"
    )
    let issuer: String = "Vercre"
    OfferCard(offer: offer, issuer: issuer)
}
