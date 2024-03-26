#ifndef _COMPLETION_H
#define _COMPLETION_H

#define MODULE_NAME	"completion"

struct completion_dev {
	struct cdev cdev;
	struct completion completion;
};

#endif