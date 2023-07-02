//
//  colobject.h
//  testfull
//
//  Created by Sébastien Keim on 02/07/2023.
//

#ifndef colobject_h
#define colobject_h


typedef void (transformer_t)(char*);

extern void make_capitalize(char*);
extern void make_upper(char*);
extern void make_lower(char*);
 
#endif /* colobject_h */
