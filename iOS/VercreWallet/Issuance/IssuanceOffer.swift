//
//  SwiftUIView.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 21/10/2024.
//

import SwiftUI

struct IssuanceOffer: View {
    @ObservedObject var core: Core

    var body: some View {
        if #available(iOS 16.0, *) {
            QRScanner()
        } else {
            // Fallback on earlier versions
        }
    }
}

#Preview {
    IssuanceOffer(core: Core())
}
