//
//  ViewController.m
//  ios
//
//  Created by liuyu on 11/12/18.
//  Copyright © 2018 liuyu. All rights reserved.
//

#import "ViewController.h"
#import "binding.h"

@interface ViewController ()

@end

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    NSLog(@"%@", @(say_hello()));
}


- (void)didReceiveMemoryWarning {
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}


@end
