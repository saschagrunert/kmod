#include <linux/init.h>
#include <linux/module.h>

MODULE_AUTHOR("Sascha Grunert <mail@saschagrunert.de>");
MODULE_LICENSE("MIT");
MODULE_VERSION("0.1.0");

#define MODULE_NAME "A simple kernel module"
MODULE_DESCRIPTION(MODULE_NAME);

extern int init(void);
extern void cleanup(void);

static int rust_module_init(void)
{
    int ret = init();
    if (ret == 0) {
        printk(KERN_INFO MODULE_NAME " initialized.\n");
    } else {
        printk(KERN_WARNING MODULE_NAME " init failed.\n");
    }
    return ret;
}

static void rust_module_exit(void)
{
    cleanup();
    printk(KERN_INFO MODULE_NAME " exiting.\n");
}

module_init(rust_module_init);
module_exit(rust_module_exit);
