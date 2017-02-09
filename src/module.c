#include <linux/module.h>
#include <linux/slab.h>

MODULE_AUTHOR("Sascha Grunert <mail@saschagrunert.de>");
MODULE_DESCRIPTION("A simple kernel module");
MODULE_LICENSE("MIT");
MODULE_VERSION("0.1.0");

// The entry and exit function
extern int init_module(void);
extern void cleanup_module(void);
