/**
 * Get current time in a specific timezone
 * @param request - The request object containing timezone parameter
 * @param env - Environment object (unused in this implementation)
 * @returns MCP-compliant result with current time information
 */
function get_current_time(request, env) {
    try {
        // console.log(`Request -> ${request.toString()}, Env -> ${env.toString()}`);
        const { timezone } = request;
        // Validate timezone parameter
        if (!timezone || typeof timezone !== 'string') {
            throw new Error('Timezone parameter is required and must be a string');
        }
        // Handle local timezone placeholder
        const targetTimezone = timezone === '{local_tz}' ?
            Intl.DateTimeFormat().resolvedOptions().timeZone : timezone;
        // Validate the timezone by trying to create a formatter
        try {
            new Intl.DateTimeFormat('en-US', { timeZone: targetTimezone });
        }
        catch (tzError) {
            throw new Error(`Invalid timezone: ${targetTimezone}`);
        }
        const now = new Date();
        // Format the time in the target timezone with explicit error handling
        let formattedTime;
        let timeOnly;
        let dateInfo;
        try {
            const formatter = new Intl.DateTimeFormat('en-US', {
                timeZone: targetTimezone,
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                hour: '2-digit',
                minute: '2-digit',
                second: '2-digit',
                hour12: false
            });
            formattedTime = formatter.format(now);
        }
        catch (formatError) {
            console.error(`Failed to format full datetime: ${formatError instanceof Error ? formatError.message : 'Unknown formatting error'}`);
            throw new Error(`Failed to format full datetime: ${formatError instanceof Error ? formatError.message : 'Unknown formatting error'}`);
        }
        try {
            const timeFormatter = new Intl.DateTimeFormat('en-US', {
                timeZone: targetTimezone,
                hourCycle: 'h23',
                hour: '2-digit',
                minute: '2-digit'
            });
            timeOnly = timeFormatter.format(now);
        }
        catch (timeError) {
            console.error(`Failed to format time: ${timeError instanceof Error ? timeError.message : 'Unknown time formatting error'}`);
            throw new Error(`Failed to format time: ${timeError instanceof Error ? timeError.message : 'Unknown time formatting error'}`);
        }
        try {
            const dateFormatter = new Intl.DateTimeFormat('en-US', {
                timeZone: targetTimezone,
                weekday: 'long',
                year: 'numeric',
                month: 'long',
                day: 'numeric'
            });
            dateInfo = dateFormatter.format(now);
        }
        catch (dateError) {
            throw new Error(`Failed to format date: ${dateError instanceof Error ? dateError.message : 'Unknown date formatting error'}`);
        }
        const result = {
            timezone: String(targetTimezone),
            current_time: String(timeOnly),
            full_datetime: String(formattedTime),
            date_info: String(dateInfo),
            iso_string: String(now.toISOString()),
            timestamp: Number(now.getTime())
        };
        return {
            content: [
                {
                    type: "text",
                    text: `Current time in ${result.timezone}: ${result.current_time} (${result.date_info})\n\nDetailed information:\n${JSON.stringify(result, null, 2)}`
                }
            ]
        };
    }
    catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        return {
            content: [
                {
                    type: "text",
                    text: `Error getting current time: ${errorMessage}`
                }
            ],
            isError: true
        };
    }
}
/**
 * Convert time between timezones
 * @param request - The request object containing conversion parameters
 * @param env - Environment object (unused in this implementation)
 * @returns MCP-compliant result with converted time information
 */
function convert_time(request, env) {
    try {
        const { source_timezone, target_timezone, time } = request;
        // Validate input parameters
        if (!source_timezone || !target_timezone || !time) {
            throw new Error('source_timezone, target_timezone, and time are required');
        }
        // Handle local timezone placeholders
        const sourceTimezone = source_timezone === '{local_tz}' ?
            Intl.DateTimeFormat().resolvedOptions().timeZone : source_timezone;
        const targetTimezone = target_timezone === '{local_tz}' ?
            Intl.DateTimeFormat().resolvedOptions().timeZone : target_timezone;
        // Validate time format (HH:MM)
        const timeRegex = /^([0-1]?[0-9]|2[0-3]):([0-5][0-9])$/;
        if (!timeRegex.test(time.trim())) {
            throw new Error(`Invalid time format: "${time}". Expected HH:MM (e.g., "14:30" or "21:15")`);
        }
        const [hours, minutes] = time.trim().split(':').map(num => parseInt(num, 10));
        // Create a date for today with the specified time
        const today = new Date();
        const sourceDate = new Date(today.getFullYear(), today.getMonth(), today.getDate(), hours, minutes, 0, 0);
        // Simple timezone conversion using Intl.DateTimeFormat
        // Create the time as if it's in the source timezone, then format it for the target timezone
        // Get the current date in both timezones to calculate offset
        const now = new Date();
        const sourceTime = new Date(now.toLocaleString('en-US', { timeZone: sourceTimezone }));
        const targetTime = new Date(now.toLocaleString('en-US', { timeZone: targetTimezone }));
        const offset = targetTime.getTime() - sourceTime.getTime();
        // Apply offset to our source date
        const convertedDate = new Date(sourceDate.getTime() + offset);
        // Format results
        const convertedTime = convertedDate.toLocaleTimeString('en-US', {
            hour12: false,
            hour: '2-digit',
            minute: '2-digit'
        });
        const fullDatetime = convertedDate.toLocaleString('en-US', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hour12: false
        });
        return {
            content: [
                {
                    type: "text",
                    text: `Time conversion: ${time} (${sourceTimezone}) → ${convertedTime} (${targetTimezone})\n\nConversion details:\n• From: ${sourceTimezone}\n• To: ${targetTimezone}\n• Original time: ${time}\n• Converted time: ${convertedTime}\n• Full datetime: ${fullDatetime}`
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Error converting time: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
// Export the functions and types for use in other modules
export { get_current_time, convert_time, };
