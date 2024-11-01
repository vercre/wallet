//
//  CounterView.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 01/11/2024.
//

import SwiftUI

struct CounterView: View {
    @ObservedObject var core: Core
    
    var body: some View {
        VStack(alignment: .leading) {
            Text("Counter")
                .font(.title)
                .fontWeight(.bold)
            Text(String(core.view.text))
                .foregroundColor(core.view.confirmed ? .black : .gray)
                .padding()
            HStack {
                ActionButton(label: "Decrease", color: .yellow) {
                    core.update(.decrement)
                }
                ActionButton(label: "Increase", color: .red) {
                    core.update(.increment)
                }
            }
            Spacer()
        }
    }
}

#Preview {
    CounterView(core: Core())
}
