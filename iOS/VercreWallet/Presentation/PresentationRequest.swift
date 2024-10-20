//
//  PresentationRequest.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 21/10/2024.
//

import SwiftUI

struct PresentationRequest: View {
    @ObservedObject var core: Core

    var body: some View {
        Text(/*@START_MENU_TOKEN@*/"Hello, World!"/*@END_MENU_TOKEN@*/)
    }
}

#Preview {
    PresentationRequest(core: Core())
}
