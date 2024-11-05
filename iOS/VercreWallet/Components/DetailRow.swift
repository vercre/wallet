//
//  DetailRow.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 05/11/2024.
//

import SwiftUI

struct DetailRow: View {
    var title: String
    var content: String
    
    var body: some View {
        VStack(alignment: .leading) {
            Text(title).font(.caption).opacity(0.5)
            Text(content)
        }.padding(.top, 12)
            .padding(.bottom, 4)
        .padding(.leading, 12)
        .padding(.trailing, 12)
    }
}

#Preview {
    DetailRow(title: "Information", content: "This is a preview of your application.")
}
