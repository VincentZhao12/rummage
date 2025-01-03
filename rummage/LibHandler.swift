//
//  LibHandler.swift
//  rummage
//
//  Created by Vincent Zhao on 1/3/25.
//

import Foundation

struct ChatResponse {
    let response: String;
    let emailIds: [String];
}

class LibHandler {
    private var ptr: UnsafeMutableRawPointer?;
    static let shared = LibHandler();
    
    private init() {
        self.ptr = rummage_make();
    }
    
    func refresh() {
        if ptr == nil {
            ptr = rummage_make();
        }
        rummage_refresh(ptr);
    }
    
    func prompt(prompt: String) -> ChatResponse? {
        if ptr == nil {
            ptr = rummage_make();
        }

        let chatResponse = rummage_prompt(ptr, strdup(prompt));
        
        defer {
            free(chatResponse.response);
        }
        
        guard let response = chatResponse.response else { return nil }

        var emailIds: [String] = []
        for i in 0..<chatResponse.len {
            if let email = chatResponse.email_ids?[Int(i)] {
                emailIds.append(String(cString: email))
            }
        }
        
        return ChatResponse(response: String(cString: response), emailIds: emailIds)
    }
    
    func killLlama() {
        if ptr == nil {
            ptr = rummage_make();
        }

        rummage_kill_llama(ptr);
    }
    
    func generateDailyUpdate() -> String? {
        if ptr == nil {
            ptr = rummage_make();
        }

        
        guard let update = rummage_generate_daily_update(ptr) else { return nil };
        defer {
            free(update)
        }
        
        return String(cString: update);
    }
    
    func destroy() {
        rummage_free(ptr);
        ptr = nil;
    }
}
