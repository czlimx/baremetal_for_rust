    .text

    .global flush_dcache_all
    .type flush_dcache_all, "function"

/*
 *	__flush_dcache_all()
 *
 *	Flush the whole D-cache.
 *
 *	Corrupted registers: x0-x7, x9-x11
 */
 flush_dcache_all:
	dsb	sy				        // ensure ordering with previous memory accesses
	mrs	x0, clidr_el1			// read clidr
	and	x3, x0, #0x7000000		// extract loc from clidr
	lsr	x3, x3, #23			    // left align loc bit field
	cbz	x3, finished			// if loc is 0, then no need to clean
	mov	x10, #0				    // start clean at cache level 0
loop1:
	add	x2, x10, x10, lsr #1	// work out 3x current cache level
	lsr	x1, x0, x2			    // extract cache type bits from clidr
	and	x1, x1, #7			    // mask of the bits for current cache only
	cmp	x1, #2				    // see what cache we have at this level
	b.lt	skip				// skip if no cache, or just i-cache
	msr	csselr_el1, x10			// select current cache level in csselr
	isb					        // isb to sych the new cssr&csidr
	mrs	x1, ccsidr_el1			// read the new ccsidr
	and	x2, x1, #7			    // extract the length of the cache lines
	add	x2, x2, #4			    // add 4 (line length offset)
	mov	x4, #0x3ff
	and	x4, x4, x1, lsr #3		// find maximum number on the way size
	clz	w5, w4				    // find bit position of way size increment
	mov	x7, #0x7fff
	and	x7, x7, x1, lsr #13		// extract max number of the index size
loop2:
	mov	x9, x4				    // create working copy of max way size
loop3:
	lsl	x6, x9, x5
	orr	x11, x10, x6			// factor way and cache number into x11
	lsl	x6, x7, x2
	orr	x11, x11, x6			// factor index number into x11
	dc	cisw, x11			    // clean & invalidate by set/way
	subs	x9, x9, #1			// decrement the way
	b.ge	loop3
	subs	x7, x7, #1			// decrement the index
	b.ge	loop2
skip:
	add	x10, x10, #2			// increment cache number
	cmp	x3, x10
	b.gt	loop1
finished:
	mov	x10, #0				    // swith back to cache level 0
	msr	csselr_el1, x10			// select current cache level in csselr
	dsb	sy
	isb
	ret
