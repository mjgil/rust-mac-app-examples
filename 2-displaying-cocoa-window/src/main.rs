extern crate cocoa;

use cocoa::base::{selector, nil, NO, id};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize, NSAutoreleasePool,
                        NSProcessInfo, NSString};

use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication,
                    NSApplicationActivateIgnoringOtherApps};

fn main() {
	unsafe {
		let _pool = NSAutoreleasePool::new(nil);

		let app = NSApp();
		app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        add_menu(&app);
        add_window();
        focus_app();
        app.run();
	}
}

/// Focus the app onscreen when launched
unsafe fn focus_app() {
    let current_app = NSRunningApplication::currentApplication(nil);
    current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
}

/// Add a window to the application instance
unsafe fn add_window() {
    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        NSRect::new(NSPoint::new(0., 0.), NSSize::new(200., 200.)),
        NSTitledWindowMask as NSUInteger,
        NSBackingStoreBuffered,
        NO
    ).autorelease();
    window.center();
    let title = NSString::alloc(nil).init_str("Hello World!");
    window.setTitle_(title);
    window.makeKeyAndOrderFront_(nil);
}

/// Add a menu to an application instance
unsafe fn add_menu(app: &id) {
    // create Menu Bar
    let menubar = NSMenu::new(nil).autorelease();
    let app_menu_item = NSMenuItem::new(nil).autorelease();
    menubar.addItem_(app_menu_item);
    app.setMainMenu_(menubar);

    // create Application menu
    let app_menu = NSMenu::new(nil).autorelease();
    let quit_prefix = NSString::alloc(nil).init_str("Quit ");
    let quit_title = quit_prefix.stringByAppendingString_(
        NSProcessInfo::processInfo(nil).processName()
    );
    let quit_action = selector("terminate:");
    let quit_key = NSString::alloc(nil).init_str("q");
    let quit_item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(
        quit_title,
        quit_action,
        quit_key
    ).autorelease();
    app_menu.addItem_(quit_item);
    app_menu_item.setSubmenu_(app_menu);
}
