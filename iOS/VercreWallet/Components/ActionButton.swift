//
//  ActionButton.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 21/10/2024.
//

import SwiftUI

struct ActionButton: View {
    var label: String
    var color: Color
    var action: () -> Void
    
    init(label: String, color: Color, action: @escaping () -> Void) {
        self.label = label
        self.color = color
        self.action = action
    }
    
    var body: some View {
        Button(action: action) {
            Text(label)
                .fontWeight(.bold)
                .font(.body)
                .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                .background(color)
                .cornerRadius(10)
                .foregroundColor(.white)
                .padding()
        }
    }
}


#Preview {
    ActionButton(label: "Action", color: .red, action: {})
}
