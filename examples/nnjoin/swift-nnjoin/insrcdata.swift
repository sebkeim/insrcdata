// generated by insrcdata version 0.3.0
import Foundation

public struct Client : Hashable {
        var cstruct : UnsafePointer<client_t>
    
    init(cstruct: UnsafePointer<client_t>){
        self.cstruct = cstruct
    }
    init(ref:clients_t){
        cstruct = client_from_clients(ref)
    }
    var clients: clients_t { client_clients(cstruct) }
    var name: String { String(cString:  client_name(cstruct)) }
    var transactions:transaction_iter_t { client_transactions(cstruct) }

} // struct Client

extension client_iter_t : Sequence, IteratorProtocol {
        public typealias Element = Client
        public mutating func next() -> Client? {
            if let ptr = client_next(&self) {
                return Client(cstruct:ptr)
            }
            return nil
        }
}


public struct Product : Hashable {
        var cstruct : UnsafePointer<product_t>
    
    init(cstruct: UnsafePointer<product_t>){
        self.cstruct = cstruct
    }
    init(ref:products_t){
        cstruct = product_from_products(ref)
    }
    var products: products_t { product_products(cstruct) }
    var name: String { String(cString:  product_name(cstruct)) }
    var transactions:transaction_iter_t { product_transactions(cstruct) }

} // struct Product

extension product_iter_t : Sequence, IteratorProtocol {
        public typealias Element = Product
        public mutating func next() -> Product? {
            if let ptr = product_next(&self) {
                return Product(cstruct:ptr)
            }
            return nil
        }
}


public struct Transaction : Hashable {
        var cstruct : UnsafePointer<transaction_t>
    
    var client : Client { Client(cstruct:transaction_client(cstruct)) }
    var product : Product { Product(cstruct:transaction_product(cstruct)) }

} // struct Transaction

extension transaction_iter_t : Sequence, IteratorProtocol {
        public typealias Element = Transaction
        public mutating func next() -> Transaction? {
            if let ptr = transaction_next(&self) {
                return Transaction(cstruct:ptr)
            }
            return nil
        }
}


