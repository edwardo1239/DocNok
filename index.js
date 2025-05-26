/**
 * @typedef {Object} Document
 * @property {string} id - Unique identifier for the document
 * @property {string} title - Title of the document
 * @property {string} content - Content of the document
 * @property {Date} createdAt - Creation date
 * @property {string[]} tags - Array of tags associated with the document
 */

/**
 * Creates a new document with the given parameters
 * @param {string} title - The title of the document
 * @param {string} content - The content of the document
 * @param {string[]} [tags=[]] - Optional tags for the document
 * @returns {Document} The created document
 * @throws {Error} If title or content are empty
 */
function createDocument(title, content, tags = []) {
    if (!title?.trim()) {
        throw new Error('Title cannot be empty');
    }
    if (!content?.trim()) {
        throw new Error('Content cannot be empty');
    }

    return {
        id: generateUniqueId(),
        title: title.trim(),
        content: content.trim(),
        createdAt: new Date(),
        tags: tags.map(tag => tag.toLowerCase())
    };
}

/**
 * Generates a unique identifier for documents
 * @private
 * @returns {string} A unique identifier
 */
function generateUniqueId() {
    return Date.now().toString(36) + Math.random().toString(36).substring(2);
}

/**
 * Searches for documents based on given criteria
 * @param {Document[]} documents - Array of documents to search through
 * @param {Object} criteria - Search criteria
 * @param {string} [criteria.title] - Optional title to search for
 * @param {string} [criteria.tag] - Optional tag to filter by
 * @param {Date} [criteria.fromDate] - Optional start date
 * @param {Date} [criteria.toDate] - Optional end date
 * @returns {Document[]} Array of documents matching the criteria
 * @example
 * const docs = searchDocuments(myDocs, {
 *     title: "Report",
 *     tag: "important",
 *     fromDate: new Date("2025-01-01")
 * });
 */
function searchDocuments(documents, criteria = {}) {
    return documents.filter(doc => {
        const matchesTitle = !criteria.title || 
            doc.title.toLowerCase().includes(criteria.title.toLowerCase());
        
        const matchesTag = !criteria.tag || 
            doc.tags.includes(criteria.tag.toLowerCase());
        
        const matchesDateRange = (!criteria.fromDate || doc.createdAt >= criteria.fromDate) &&
            (!criteria.toDate || doc.createdAt <= criteria.toDate);

        return matchesTitle && matchesTag && matchesDateRange;
    });
}

/**
 * Formats a document for display or export
 * @param {Document} document - The document to format
 * @param {Object} [options] - Formatting options
 * @param {boolean} [options.includeId=false] - Whether to include the document ID
 * @param {boolean} [options.includeTags=true] - Whether to include tags
 * @param {string} [options.dateFormat='ISO'] - Date format ('ISO' | 'local' | 'relative')
 * @returns {Object} Formatted document
 * @throws {Error} If document is invalid or date format is unsupported
 */
function formatDocument(document, options = {}) {
    const {
        includeId = false,
        includeTags = true,
        dateFormat = 'ISO'
    } = options;

    if (!document?.title || !document?.content) {
        throw new Error('Invalid document');
    }

    const formatted = {
        title: document.title,
        content: document.content,
        date: formatDate(document.createdAt, dateFormat)
    };

    if (includeId) {
        formatted.id = document.id;
    }

    if (includeTags) {
        formatted.tags = [...document.tags];
    }

    return formatted;
}

/**
 * Formats a date according to the specified format
 * @private
 * @param {Date} date - The date to format
 * @param {string} format - The desired format ('ISO' | 'local' | 'relative')
 * @returns {string} Formatted date string
 * @throws {Error} If format is unsupported
 */
function formatDate(date, format) {
    switch (format) {
        case 'ISO':
            return date.toISOString();
        case 'local':
            return date.toLocaleString();
        case 'relative':
            return getRelativeTimeString(date);
        default:
            throw new Error(`Unsupported date format: ${format}`);
    }
}